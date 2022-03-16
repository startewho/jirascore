use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "user_issue")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub issue_id: i32,
    #[sea_orm(primary_key)]
    pub user_id: i32,
    pub score: f32,
    pub update_time: DateTime,
    pub comment: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
