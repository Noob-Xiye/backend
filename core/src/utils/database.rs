use sea_orm::{DatabaseConnection, Database, DbErr};
use sea_orm::ConnectOptions;
use std::time::Duration;
use tracing::log::info;

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