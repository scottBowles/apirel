//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "race_race_ability_score_increases")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub race_id: i64,
    pub abilityscoreincrease_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::race_abilityscoreincrease::Entity",
        from = "Column::AbilityscoreincreaseId",
        to = "super::race_abilityscoreincrease::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    RaceAbilityscoreincrease,
    #[sea_orm(
        belongs_to = "super::race_race::Entity",
        from = "Column::RaceId",
        to = "super::race_race::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    RaceRace,
}

impl Related<super::race_abilityscoreincrease::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RaceAbilityscoreincrease.def()
    }
}

impl Related<super::race_race::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RaceRace.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}