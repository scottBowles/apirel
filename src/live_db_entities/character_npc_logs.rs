//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "character_npc_logs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub npc_id: i64,
    pub gamelog_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::character_npc::Entity",
        from = "Column::NpcId",
        to = "super::character_npc::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    CharacterNpc,
    #[sea_orm(
        belongs_to = "super::nucleus_gamelog::Entity",
        from = "Column::GamelogId",
        to = "super::nucleus_gamelog::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    NucleusGamelog,
}

impl Related<super::character_npc::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CharacterNpc.def()
    }
}

impl Related<super::nucleus_gamelog::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::NucleusGamelog.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}