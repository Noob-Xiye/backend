use redis::aio::ConnectionManager;
use redis::{Client, RedisResult};
use once_cell::sync::OnceCell;
use crate::core::settings::Config;
use tracing::info;

// 使用 OnceCell 来存储 Redis 连接池，确保全局唯一
static REDIS_POOL: OnceCell<ConnectionManager> = OnceCell::new();

/// 初始化 Redis 连接池
pub async fn init_redis(config: &Config) -> RedisResult<()> {
    let redis_url = &config.redis.url;
    let client = Client::open(redis_url)?;

    // TODO: 根据 config.toml 配置设置密码, db, pool_size, timeout, max_retries
    // redis-rs crate 的 ConnectionManager 不直接支持所有这些参数，可能需要更底层的连接管理或不同的库
    // 这里先使用 ConnectionManager 作为基本实现
    let pool = ConnectionManager::new(client).await?;

    REDIS_POOL.set(pool).map_err(|_| redis::RedisError::from((redis::ErrorKind::ClientConfigurationError, "Redis pool already initialized.")))
}

/// 获取 Redis 连接池
pub fn get_redis_pool() -> RedisResult<&'static ConnectionManager> {
    REDIS_POOL.get().ok_or(redis::RedisError::from((redis::ErrorKind::ClientConfigurationError, "Redis pool not initialized.")))
}

// TODO: 添加更详细的连接池配置和错误处理

pub struct RedisPool {
    client: Client,
}

impl RedisPool {
    pub async fn new(redis_url: &str) -> Result<Self, redis::RedisError> {
        info!("Connecting to Redis...");
        let client = Client::open(redis_url)?;
        // 您可以在此处测试连接，例如: client.get_connection().await?
        Ok(Self { client })
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}

// 示例用法 (可选，仅用于演示)
/*
pub async fn get_value(
    pool: &RedisPool,
    key: &str,
) -> Result<Option<String>, redis::RedisError> {
    let mut conn = pool.client().get_async_connection().await?;
    conn.get(key).await
}
*/ 