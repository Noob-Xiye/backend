use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64, // Snowflake ID
    pub customer_id: i64, // 客户外键
    pub product_id: i64,  // 产品外键
    pub order_amount: f64,
    pub payment_method: String,         // 例如: "cregis", "trc20", "erc20"
    pub payment_record_id: Option<i64>, // 可选的 payment_record 外键
    pub status: String,                 // 例如: "pending", "paid", "cancelled", "completed"
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = super::customer::Entity, from = column::CustomerId, to = super::customer::Column::Id)]
    Customer, // 关联到 Customer 实体
    #[sea_orm(belongs_to = super::product::Entity, from = column::ProductId, to = super::product::Column::Id)]
    Product, // 关联到 Product 实体
    #[sea_orm(belongs_to = super::payment_record::Entity, from = column::PaymentRecordId, to = super::payment_record::Column::Id)]
    PaymentRecord, // 关联到 PaymentRecord 实体
}

impl ActiveModelBehavior for ActiveModel {}
