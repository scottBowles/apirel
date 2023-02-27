//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "us_gaz")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub seq: Option<i32>,
    #[sea_orm(column_type = "Text", nullable)]
    pub word: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub stdword: Option<String>,
    pub token: Option<i32>,
    pub is_custom: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
