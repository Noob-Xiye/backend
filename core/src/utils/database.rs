use sea_orm::{DatabaseConnection, Database, DbErr};
use sea_orm::ConnectOptions;
use std::time::Duration;
use tracing::log::info;
use once_cell::sync::OnceCell;
use crate::core::settings::Config;

// 使用 OnceCell 来存储数据库连接池，确保全局唯一
static DATABASE_POOL: OnceCell<DatabaseConnection> = OnceCell::new();

/// 初始化数据库连接池
pub async fn init_database(config: &Config) -> Result<(), DbErr> {
    let database_url = &config.database.url;
    let pool = Database::connect(database_url).await?;

    // 设置连接池参数 (根据 config.toml 配置)
    // Note: Sea-ORM连接池参数通常在连接URL或Database::connect选项中设置，这里简化处理

    DATABASE_POOL.set(pool).map_err(|_| DbErr::Custom("Database pool already initialized.".to_string()))
}

/// 获取数据库连接池
pub fn get_database_pool() -> Result<&'static DatabaseConnection, DbErr> {
    DATABASE_POOL.get().ok_or(DbErr::Custom("Database pool not initialized.".to_string()))
}

pub async fn establish_connection(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(database_url.to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);

    info!("Establishing database connection...");
    let db = Database::connect(opt).await?;

    Ok(db)
} 