use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "logger_confs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,  // e.g., "default"
    pub level: String, // e.g., "info", "debug", "error"
    pub output: String, // e.g., "stdout", "file", "database"
                       // Add other relevant configuration fields
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
