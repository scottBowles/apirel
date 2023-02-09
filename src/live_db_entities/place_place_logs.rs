//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "place_place_logs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub place_id: i64,
    pub gamelog_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::nucleus_gamelog::Entity",
        from = "Column::GamelogId",
        to = "super::nucleus_gamelog::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    NucleusGamelog,
    #[sea_orm(
        belongs_to = "super::place_place::Entity",
        from = "Column::PlaceId",
        to = "super::place_place::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    PlacePlace,
}

impl Related<super::nucleus_gamelog::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::NucleusGamelog.def()
    }
}

impl Related<super::place_place::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PlacePlace.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}