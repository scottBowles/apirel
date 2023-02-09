//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "item_artifact")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub created: DateTimeWithTimeZone,
    pub updated: DateTimeWithTimeZone,
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    #[sea_orm(unique)]
    pub slug: String,
    pub thumbnail_id: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub notes: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub markdown_notes: Option<String>,
    pub lock_time: Option<DateTimeWithTimeZone>,
    pub lock_user_id: Option<i64>,
    pub image_ids: Vec<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::item_artifact_items::Entity")]
    ItemArtifactItems,
    #[sea_orm(has_many = "super::item_artifact_logs::Entity")]
    ItemArtifactLogs,
    #[sea_orm(
        belongs_to = "super::nucleus_user::Entity",
        from = "Column::LockUserId",
        to = "super::nucleus_user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    NucleusUser,
}

impl Related<super::item_artifact_items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemArtifactItems.def()
    }
}

impl Related<super::item_artifact_logs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemArtifactLogs.def()
    }
}

impl Related<super::nucleus_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::NucleusUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}