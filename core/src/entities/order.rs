use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64, // Snowflake ID
    pub customer_id: i64, // Foreign key to customer
    pub product_id: i64,  // Foreign key to product
    pub order_amount: f64,
    pub payment_method: String,         // e.g., "cregis", "trc20", "erc20"
    pub payment_record_id: Option<i64>, // Optional foreign key to payment_record
    pub status: String,                 // e.g., "pending", "paid", "cancelled", "completed"
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = super::customer::Entity, from = column::CustomerId, to = super::customer::Column::Id)]
    Customer,
    #[sea_orm(belongs_to = super::product::Entity, from = column::ProductId, to = super::product::Column::Id)]
    Product,
    #[sea_orm(belongs_to = super::payment_record::Entity, from = column::PaymentRecordId, to = super::payment_record::Column::Id)]
    PaymentRecord,
}

impl ActiveModelBehavior for ActiveModel {}
