//! `SeaORM` Entity for the started missions.

use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "missions_started")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: Uuid,

    #[sea_orm(primary_key, auto_increment = false)]
    pub mission_id: i16,

    pub mission_progress: i32,

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
