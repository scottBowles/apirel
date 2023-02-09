//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "character_tool")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::character_inventorytool::Entity")]
    CharacterInventorytool,
}

impl Related<super::character_inventorytool::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CharacterInventorytool.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
