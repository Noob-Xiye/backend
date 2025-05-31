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
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("config"))
            // Add in the current environment file
            // E.g. `development.toml`, `production.toml`
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            // Add in settings from environment variables (with a prefix of APP)
            // E.g. `APP_DEBUG=1 would reference a `debug` key
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        // your own complex struct.
        s.try_deserialize()
    }
}
