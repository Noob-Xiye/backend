use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "payment_records")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64, // Snowflake ID
    pub order_id: i64,          // 订单外键
    pub payment_method: String, // 例如: "cregis", "trc20", "erc20"
    pub transaction_id: String, // 支付网关的交易 ID
    pub amount: f64,
    pub status: String, // 例如: "pending", "success", "failed"
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub paid_at: Option<chrono::NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = super::order::Entity, from = column::OrderId, to = super::order::Column::Id)]
    Order, // 关联到 Order 实体
}

impl ActiveModelBehavior for ActiveModel {}
