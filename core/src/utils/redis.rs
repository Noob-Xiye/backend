use redis::{Client, RedisError, AsyncCommands};
use tracing::info;

pub struct RedisPool {
    client: Client,
}

impl RedisPool {
    pub async fn new(redis_url: &str) -> Result<Self, RedisError> {
        info!("Connecting to Redis...");
        let client = Client::open(redis_url)?;
        // You might want to test the connection here, e.g., client.get_connection().await?
        Ok(Self { client })
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}

// Example usage (optional, for demonstration)
/*
pub async fn get_value(
    pool: &RedisPool,
    key: &str,
) -> Result<Option<String>, RedisError> {
    let mut conn = pool.client().get_async_connection().await?;
    conn.get(key).await
}
*/ 