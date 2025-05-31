use snowflake::SnowflakeIdGenerator;
use std::sync::Mutex;

// 在实际应用中，worker_id 和 node_id 应该进行配置。
// 为了简单起见，这里使用硬编码值。
lazy_static::lazy_static! {
    static ref SNOWFLAKE_ID_GENERATOR: Mutex<SnowflakeIdGenerator> =
        Mutex::new(SnowflakeIdGenerator::new(1, 1));
}

pub fn generate_snowflake_id() -> i64 {
    SNOWFLAKE_ID_GENERATOR.lock().unwrap().next_id()
}
