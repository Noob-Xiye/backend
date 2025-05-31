use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "web_profiles")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub site_name: String,
    pub site_version: String,
    pub site_logo_url: Option<String>,
    pub smtp_server: Option<String>,
    pub smtp_port: Option<i32>,
    pub smtp_user: Option<String>,
    pub smtp_password: Option<String>,
    pub payment_callback_url: Option<String>,
    // Add other relevant website settings
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
