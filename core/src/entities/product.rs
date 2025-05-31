use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "products")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64, // Snowflake ID
    pub merchant_id: i64, // 商家外键
    pub name: String,
    pub price: f64,
    pub image_url: Option<String>,
    pub description_url: Option<String>,
    pub listed_at: chrono::NaiveDateTime,
    pub version: String,
    pub updated_at: chrono::NaiveDateTime,
    pub download_url: Option<String>,
    pub sales_count: i64,
    pub status: String, // 例如: "listed", "unlisted"
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = super::merchant::Entity, from = column::MerchantId, to = super::merchant::Column::Id)]
    Merchant, // 关联到 Merchant 实体
}

impl ActiveModelBehavior for ActiveModel {}
