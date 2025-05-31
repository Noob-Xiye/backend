use snowflake::SnowflakeIdGenerator;
use std::sync::Mutex;

// In a real application, the worker_id and node_id should be configured.
// For simplicity, we use hardcoded values here.
lazy_static::lazy_static! {
    static ref SNOWFLAKE_ID_GENERATOR: Mutex<SnowflakeIdGenerator> =
        Mutex::new(SnowflakeIdGenerator::new(1, 1));
}

pub fn generate_snowflake_id() -> i64 {
    SNOWFLAKE_ID_GENERATOR.lock().unwrap().next_id()
}
