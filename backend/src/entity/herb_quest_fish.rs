//! `SeaORM` Entity for the fishes of the current Herb quest.

use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "herb_quest_fish")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub quest_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub fish_id: i32,
    pub amount: i32,
    #[sea_orm(
        belongs_to,
        from = "quest_id",
        to = "quest_id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    pub herb_quest: HasOne<super::herb_quest::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
