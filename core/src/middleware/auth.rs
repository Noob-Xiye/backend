use salvo::prelude::*;
use salvo::http::StatusError;
use anyhow::Result;
use crate::error::AppError;
use async_trait::async_trait;

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