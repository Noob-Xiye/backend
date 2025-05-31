use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "logs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64, // Snowflake ID
    pub user_type: String,    // e.g., "customer", "merchant", "admin", "system"
    pub user_id: Option<i64>, // Optional user ID (Snowflake ID)
    pub action: String,       // e.g., "login", "register", "create_product", "update_settings"
    pub details: Option<String>,
    pub ip_address: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
