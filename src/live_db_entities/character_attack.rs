//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "character_attack")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub attack_bonus: i32,
    pub damage: String,
    pub damage_type: String,
    pub range: i32,
    #[sea_orm(column_type = "Text")]
    pub properties: String,
    pub ability_options: Vec<String>,
    pub character_id: i64,
    pub proficiency_needed_id: Option<i64>,
    pub weapon_id: i64,
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
        belongs_to = "super::character_proficiency::Entity",
        from = "Column::ProficiencyNeededId",
        to = "super::character_proficiency::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    CharacterProficiency,
    #[sea_orm(
        belongs_to = "super::item_item::Entity",
        from = "Column::WeaponId",
        to = "super::item_item::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    ItemItem,
}

impl Related<super::character_character::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CharacterCharacter.def()
    }
}

impl Related<super::character_proficiency::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CharacterProficiency.def()
    }
}

impl Related<super::item_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemItem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}