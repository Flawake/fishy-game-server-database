//! `SeaORM` Entity for the single, globally shared current Herb quest.

use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "herb_quest")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub quest_id: Uuid,
    pub area_id: i32,
    pub reward_coins: i32,
    pub next_move_at: DateTimeWithTimeZone,
    pub created_at: DateTimeWithTimeZone,
    #[sea_orm(has_many)]
    pub herb_quest_fishes: HasMany<super::herb_quest_fish::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
