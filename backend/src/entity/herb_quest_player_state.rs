//! `SeaORM` Entity tracking each player's Herb quest progress
//! (last accepted and last completed quest, both by quest UUID).

use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "herb_quest_player_state")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: Uuid,
    pub last_completed_quest_id: Option<Uuid>,
    pub last_accepted_quest_id: Option<Uuid>,
    #[sea_orm(
        belongs_to,
        from = "user_id",
        to = "user_id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    pub users: HasOne<super::users::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
