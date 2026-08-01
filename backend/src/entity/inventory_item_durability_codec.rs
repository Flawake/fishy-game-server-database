//! `SeaORM` Entity

use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "inventory_item_durability_codec")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub item_uuid: Uuid,
    pub current_durability: i32,
}

impl ActiveModelBehavior for ActiveModel {}
