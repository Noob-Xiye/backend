use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub keep_alive: u64,
    pub max_connections: usize,
    pub tcp_nodelay: bool,
    pub tcp_keepalive: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
    pub idle_timeout: u64,
    pub max_lifetime: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub url: String,
    pub password: Option<String>,
    pub db: u8,
    pub pool_size: u32,
    pub timeout: u64,
    pub max_retries: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SessionConfig {
    pub name: String,
    pub expires: i64,
    pub path: String,
    pub domain: Option<String>,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MonitoringConfig {
    pub sentry_dsn: Option<String>,
    pub prometheus_port: u16,
    pub metrics_path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SecurityConfig {
    pub rate_limit: u32,
    pub trust_proxy: bool,
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
    pub exposed_headers: Vec<String>,
    pub max_age: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CacheConfig {
    pub default_ttl: u64,
    pub max_size: u64,
    pub cleanup_interval: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SnowflakeConfig {
    pub worker_id: i32,
    pub datacenter_id: i32,
    pub epoch: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub session: SessionConfig,
    pub monitoring: MonitoringConfig,
    pub security: SecurityConfig,
    pub cache: CacheConfig,
    pub snowflake: SnowflakeConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            // 首先合并“默认”配置文件
            .add_source(File::with_name("config"))
            // 添加当前环境对应的配置文件
            // 例如: `development.toml`, `production.toml`
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            // 添加环境变量中的设置 (以 APP 为前缀)
            // 例如: `APP_DEBUG=1` 将引用 `debug` 键
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;

        // 您可以将整个配置反序列化 (并因此“冻结”) 到您自己的复杂结构中。
        s.try_deserialize()
    }
}
