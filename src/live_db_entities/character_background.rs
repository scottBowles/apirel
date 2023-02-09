//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "character_background")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    #[sea_orm(column_type = "Text")]
    pub equipment_options: String,
    #[sea_orm(column_type = "Text")]
    pub suggested_characteristics: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::character_background_features::Entity")]
    CharacterBackgroundFeatures,
    #[sea_orm(has_many = "super::character_background_languages::Entity")]
    CharacterBackgroundLanguages,
    #[sea_orm(has_many = "super::character_background_skill_proficiencies::Entity")]
    CharacterBackgroundSkillProficiencies,
    #[sea_orm(has_many = "super::character_character::Entity")]
    CharacterCharacter,
}

impl Related<super::character_background_features::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CharacterBackgroundFeatures.def()
    }
}

impl Related<super::character_background_languages::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CharacterBackgroundLanguages.def()
    }
}

impl Related<super::character_background_skill_proficiencies::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CharacterBackgroundSkillProficiencies.def()
    }
}

impl Related<super::character_character::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CharacterCharacter.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
