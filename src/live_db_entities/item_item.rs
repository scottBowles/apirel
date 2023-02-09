//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "item_item")]
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
    pub markdown_notes: Option<String>,
    pub lock_time: Option<DateTimeWithTimeZone>,
    pub lock_user_id: Option<i64>,
    pub image_ids: Vec<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::character_attack::Entity")]
    CharacterAttack,
    #[sea_orm(has_many = "super::character_inventoryarmor::Entity")]
    CharacterInventoryarmor,
    #[sea_orm(has_many = "super::character_inventoryequipment::Entity")]
    CharacterInventoryequipment,
    #[sea_orm(has_many = "super::character_inventoryweapon::Entity")]
    CharacterInventoryweapon,
    #[sea_orm(has_one = "super::item_armortraits::Entity")]
    ItemArmortraits,
    #[sea_orm(has_many = "super::item_artifact_items::Entity")]
    ItemArtifactItems,
    #[sea_orm(has_one = "super::item_equipmenttraits::Entity")]
    ItemEquipmenttraits,
    #[sea_orm(has_many = "super::item_item_logs::Entity")]
    ItemItemLogs,
    #[sea_orm(has_one = "super::item_weapontraits::Entity")]
    ItemWeapontraits,
    #[sea_orm(
        belongs_to = "super::nucleus_user::Entity",
        from = "Column::LockUserId",
        to = "super::nucleus_user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    NucleusUser,
}

impl Related<super::character_attack::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CharacterAttack.def()
    }
}

impl Related<super::character_inventoryarmor::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CharacterInventoryarmor.def()
    }
}

impl Related<super::character_inventoryequipment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CharacterInventoryequipment.def()
    }
}

impl Related<super::character_inventoryweapon::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CharacterInventoryweapon.def()
    }
}

impl Related<super::item_armortraits::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemArmortraits.def()
    }
}

impl Related<super::item_artifact_items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemArtifactItems.def()
    }
}

impl Related<super::item_equipmenttraits::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemEquipmenttraits.def()
    }
}

impl Related<super::item_item_logs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemItemLogs.def()
    }
}

impl Related<super::item_weapontraits::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemWeapontraits.def()
    }
}

impl Related<super::nucleus_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::NucleusUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
