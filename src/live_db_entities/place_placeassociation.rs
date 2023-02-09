//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "place_placeassociation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(column_type = "Text")]
    pub notes: String,
    pub association_id: i64,
    pub place_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::association_association::Entity",
        from = "Column::AssociationId",
        to = "super::association_association::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    AssociationAssociation,
    #[sea_orm(
        belongs_to = "super::place_place::Entity",
        from = "Column::PlaceId",
        to = "super::place_place::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    PlacePlace,
}

impl Related<super::association_association::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AssociationAssociation.def()
    }
}

impl Related<super::place_place::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PlacePlace.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}