use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "merchants")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64, // Snowflake ID
    pub account: String,
    pub password_hash: String,
    pub nickname: String,
    pub avatar_url: Option<String>,
    pub role: String,   // e.g., "merchant"
    pub status: String, // e.g., "active", "inactive"
    pub erc_wallet_address: Option<String>,
    pub trc_wallet_address: Option<String>,
    pub cregis_wallet_id: Option<String>,
    pub last_login_time: Option<chrono::NaiveDateTime>,
    pub last_login_ip: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub registered_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
