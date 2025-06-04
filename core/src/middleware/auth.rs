use salvo::prelude::*;
use salvo::http::StatusError;
use anyhow::Result;
use crate::error::AppError;
use async_trait::async_trait;
use crate::core::error::{AppError, AppResult, error_list::{UNAUTHORIZED, FORBIDDEN}};

// 定义您的用户标识符类型，例如，用户 ID 或会话 token
#[derive(Clone, Debug)]
pub struct AuthenticatedUser {
    pub user_id: i64,
    pub role: String, // 例如: "admin", "customer", "merchant"
    // 添加其他相关的用户信息
}

// 授权逻辑 trait
pub trait Authorize {
    fn has_permission(&self, required_permission: &str) -> bool;
}

impl Authorize for AuthenticatedUser {
    fn has_permission(&self, required_permission: &str) -> bool {
        // 在这里实现您的权限检查逻辑
        // 这是一个占位符实现
        match self.role.as_str() {
            "super_admin" => true, // 超级管理员拥有所有权限
            "admin" => required_permission.starts_with("admin:"),
            "merchant" => required_permission.starts_with("merchant:"),
            "customer" => required_permission.starts_with("customer:"),
            _ => false,
        }
    }
}

pub struct AuthGuard {
    // 您可能需要在这里访问数据库或会话存储
}

impl AuthGuard {
    pub fn new() -> Self {
        AuthGuard {}
    }
}

#[async_trait]
impl Handler for AuthGuard {
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        // 在这里实现您的身份验证逻辑
        // 示例: 检查会话或 JWT token
        // 如果已认证，将 AuthenticatedUser 存储在 Depot 中
        // 如果未认证，返回错误并调用 ctrl.skip_rest()

        // 占位符: 假设身份验证成功并设置一个虚拟用户
        let authenticated_user = AuthenticatedUser {
            user_id: 1, // 虚拟用户 ID
            role: "admin".to_string(), // 虚拟角色
        };
        depot.insert::<AuthenticatedUser>(authenticated_user);

        // 如果身份验证失败，取消注释以下行:
        // res.render(StatusError::unauthorized().brief("Authentication required"));
        // ctrl.skip_rest();
    }
}

// 帮助函数从 Depot 获取已认证用户
pub fn get_authenticated_user(depot: &Depot) -> Result<&AuthenticatedUser, AppError> {
    depot.get::<AuthenticatedUser>().ok_or_else(|| AppError::AuthError { code: 4001, message: "Authentication required".to_string() })
}

// 用于检查权限的属性
#[salvo::handler]
pub async fn check_permission(
    required_permission: &str,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    let authenticated_user = match get_authenticated_user(depot) {
        Ok(user) => user,
        Err(e) => {
            res.render(StatusError::unauthorized().brief(e.to_string()));
            ctrl.skip_rest();
            return;
        }
    };

    if !authenticated_user.has_permission(required_permission) {
        res.render(StatusError::forbidden().brief("Insufficient permissions"));
        ctrl.skip_rest();
    }
}

#[derive(Clone)]
pub struct AuthMiddleware {
    // TODO: 添加配置字段和权限验证逻辑
}

impl AuthMiddleware {
    pub fn new() -> Self {
        AuthMiddleware { /* TODO: 初始化配置 */ }
    }
}

#[async_trait]
impl Handler for AuthMiddleware {
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        // TODO: 实现 token 解析和验证
        let is_authenticated = false; // 占位符
        let has_permission = false; // 占位符

        if !is_authenticated {
            let error = AppError::new(UNAUTHORIZED as i32);
             res.render(salvo::http::Status::from(error.status_code()).with_body(serde_json::to_string(&crate::core::model::response::UnifiedResponse::<()>{
                 code: error.code(),
                 msg: error.message(),
                 data: None,
             }).unwrap()));
            ctrl.skip_rest();
            return;
        }

        // TODO: 如果需要权限验证，实现权限检查
        if !has_permission {
             let error = AppError::new(FORBIDDEN as i32);
              res.render(salvo::http::Status::from(error.status_code()).with_body(serde_json::to_string(&crate::core::model::response::UnifiedResponse::<()>{
                  code: error.code(),
                  msg: error.message(),
                  data: None,
              }).unwrap()));
             ctrl.skip_rest();
             return;
         }

        ctrl.call_next(req, depot, res, ctrl).await;
    }
}

// TODO: 添加 token 生成和验证的辅助函数 