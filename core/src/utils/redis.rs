use redis::{Client, RedisError, AsyncCommands};
use tracing::info;

pub struct RedisPool {
    client: Client,
}

impl RedisPool {
    pub async fn new(redis_url: &str) -> Result<Self, RedisError> {
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
) -> Result<Option<String>, RedisError> {
    let mut conn = pool.client().get_async_connection().await?;
    conn.get(key).await
}
*/ 