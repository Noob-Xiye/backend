use salvo::prelude::*;
use salvo::session::{SessionMiddleware, RedisStore};
use crate::settings::AppConfig;
use crate::utils::redis::RedisPool;
use anyhow::Result as AnyhowResult;
use crate::error::AppError;

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