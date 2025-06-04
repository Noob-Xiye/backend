use crate::settings::AppConfig;
use salvo::extra::csrf::CsrfGuard;
use salvo::prelude::*;
use salvo::http::Method;
use crate::core::error::{AppError, AppResult, error_list::CSRF_TOKEN_VALIDATION_FAILED};

pub fn csrf_middleware(config: &AppConfig) -> CsrfGuard {
    // 在实际应用中，密钥应该安全生成和存储。
    // 为了简单起见，这里使用硬编码值。
    let secret = "this_is_a_very_secret_key_change_it_in_production"; // 在生产环境中替换为真实的密钥

    CsrfGuard::new()
        .with_secret(secret.as_bytes())
        .with_httponly(config.session.http_only)
        .with_cookie_name("csrf_token")
        .with_header_name("X-CSRF-Token")
}

#[derive(Clone)]
pub struct CsrfMiddleware {
    // TODO: 添加配置字段
}

impl CsrfMiddleware {
    pub fn new() -> Self {
        CsrfMiddleware { /* TODO: 初始化配置 */ }
    }
}

#[async_trait]
impl Handler for CsrfMiddleware {
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        // TODO: 实现 CSRF token 的生成和验证逻辑
        // 通常对于 GET, HEAD, OPTIONS, TRACE 方法不需要验证
        // 对于 POST, PUT, DELETE 等方法，需要检查请求头或表单中是否存在有效的 CSRF token
        
        if req.method() != Method::GET && req.method() != Method::HEAD && req.method() != Method::OPTIONS && req.method() != Method::TRACE {
            // TODO: 验证 token
            let is_valid = false; // 占位符
            
            if !is_valid {
                let error = AppError::new(CSRF_TOKEN_VALIDATION_FAILED as i32);
                res.render(salvo::http::Status::from(error.status_code()).with_body(serde_json::to_string(&crate::core::model::response::UnifiedResponse::<()>{
                    code: error.code(),
                    msg: error.message(),
                    data: None,
                }).unwrap()));
                ctrl.skip_rest();
                return;
            }
        }

        ctrl.call_next(req, depot, res, ctrl).await;
    }
}

// TODO: 添加 CSRF token 生成和验证的辅助函数
