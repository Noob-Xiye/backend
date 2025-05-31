use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "admins")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64, // Using i64 for Snowflake ID
    pub account: String,
    pub password_hash: String,
    pub nickname: String,
    pub avatar_url: Option<String>,
    pub role: String,   // e.g., "admin", "super_admin"
    pub status: String, // e.g., "active", "inactive"
    pub last_login_time: Option<chrono::NaiveDateTime>,
    pub last_login_ip: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
