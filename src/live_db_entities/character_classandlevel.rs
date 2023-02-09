//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "character_classandlevel")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub level: i32,
    pub character_id: i64,
    pub character_class_id: i64,
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
    #[sea_orm(
        belongs_to = "super::character_characterclass::Entity",
        from = "Column::CharacterClassId",
        to = "super::character_characterclass::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    CharacterCharacterclass,
}

impl Related<super::character_character::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CharacterCharacter.def()
    }
}

impl Related<super::character_characterclass::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CharacterCharacterclass.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}