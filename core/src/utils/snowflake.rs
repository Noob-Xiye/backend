use crate::core::settings::Config;
use once_cell::sync::OnceCell;
use snowflake::{Error as SnowflakeError, SnowflakeIdGenerator};
use std::sync::Mutex;

// 使用 OnceCell 和 Mutex 来存储雪花 ID 生成器，确保全局唯一和线程安全
static SNOWFLAKE_GENERATOR: OnceCell<Mutex<SnowflakeIdGenerator>> = OnceCell::new();

/// 初始化雪花 ID 生成器
pub fn init_snowflake(config: &Config) -> Result<(), SnowflakeError> {
    let generator = SnowflakeIdGenerator::new(
        config.snowflake.worker_id as i32,
        config.snowflake.datacenter_id as i32,
    );
    SNOWFLAKE_GENERATOR
        .set(Mutex::new(generator))
        .map_err(|_| SnowflakeError::InvalidSystemTime)
}

/// 生成一个新的雪花 ID
pub fn generate_id() -> Result<i64, SnowflakeError> {
    let generator = SNOWFLAKE_GENERATOR
        .get()
        .ok_or(SnowflakeError::InvalidSystemTime)?;
    let mut generator = generator.lock().unwrap();
    Ok(generator.real_time_generate())
}

// TODO: 处理 Mutex 锁定失败的情况，考虑更高级的并发控制
