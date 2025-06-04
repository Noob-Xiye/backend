use salvo::prelude::*;
use salvo::session::{SessionMiddleware, RedisStore};
use crate::settings::AppConfig;
use crate::utils::redis::RedisPool;
use anyhow::Result as AnyhowResult;
use crate::error::AppError;
use salvo::session::Session;
use crate::core::settings::Config;

pub async fn session_middleware(config: &AppConfig, redis_pool: &RedisPool) -> Result<SessionMiddleware, AppError> {
    let session_config = config.session.clone();

    // 使用 Redis 存储会话
    let store = RedisStore::new(redis_pool.client().clone()).await?;

    Ok(SessionMiddleware::new(store)
        .with_name(session_config.name)
        .with_duration(salvo::session::Duration::seconds(session_config.expires))
        .with_path(session_config.path)
        .with_domain(session_config.domain.clone())
        .with_secure(session_config.secure)
        .with_http_only(session_config.http_only)
        .with_same_site(match session_config.same_site.to_lowercase().as_str() {
            "strict" => salvo::session::SameSite::Strict,
            "lax" => salvo::session::SameSite::Lax,
            "none" => salvo::session::SameSite::None,
            _ => salvo::session::SameSite::Lax, // 默认 Lax
        }))
}

#[derive(Clone)]
pub struct SessionMiddleware {
    // TODO: 添加配置字段和 session 存储客户端
}

impl SessionMiddleware {
    pub fn new(config: &Config) -> Self {
        // TODO: 根据配置初始化 session 存储
        SessionMiddleware { /* TODO: 初始化存储 */ }
    }
}

#[async_trait]
impl Handler for SessionMiddleware {
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        // TODO: 实现 session 的加载、保存和更新逻辑
        // 将 session 存储在 depot 中
        // depot.insert("session", session);

        ctrl.call_next(req, depot, res, ctrl).await;

        // TODO: 在请求处理完成后保存 session
    }
}

// TODO: 添加 session 存储相关的实现 (如 Redis 存储) 