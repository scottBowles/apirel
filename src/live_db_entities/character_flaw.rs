//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "character_flaw")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub text: String,
    pub character_id: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::character_character::Entity",
        from = "Column::CharacterId",
        to = "super::character_character::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    CharacterCharacter,
}

impl Related<super::character_character::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CharacterCharacter.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
