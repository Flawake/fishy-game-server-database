use rocket::async_trait;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseTransaction, DbErr,
    EntityTrait, QueryFilter,
};
use uuid::Uuid;

use crate::entity::{missions_completed, missions_started};

#[async_trait]
pub trait MissionRepository: Send + Sync {
    async fn start_mission(
        &self,
        tx: &DatabaseTransaction,
        user_id: Uuid,
        mission_id: i16,
    ) -> Result<(), DbErr>;

    async fn progress_mission(
        &self,
        tx: &DatabaseTransaction,
        user_id: Uuid,
        mission_id: i16,
        new_progress: i32,
    ) -> Result<(), DbErr>;

    async fn complete_mission(
        &self,
        tx: &DatabaseTransaction,
        user_id: Uuid,
        mission_id: i16,
    ) -> Result<(), DbErr>;
}

#[derive(Debug, Clone)]
pub struct MissionRepositoryImpl;

impl MissionRepositoryImpl {
    pub fn new() -> Self {
        Self
    }

    async fn delete_started_mission(
        &self,
        tx: &DatabaseTransaction,
        user_id: Uuid,
        mission_id: i16,
    ) -> Result<(), DbErr> {
        missions_started::Entity::delete_many()
            .filter(
                Condition::all()
                    .add(missions_started::Column::UserId.eq(user_id))
                    .add(missions_started::Column::MissionId.eq(mission_id)),
            )
            .exec(tx)
            .await?;

        Ok(())
    }

    async fn add_completed_mission(
        &self,
        tx: &DatabaseTransaction,
        user_id: Uuid,
        mission_id: i16,
    ) -> Result<(), DbErr> {
        missions_completed::ActiveModel {
            user_id: Set(user_id),
            mission_id: Set(mission_id),
        }
        .insert(tx)
        .await?;

        Ok(())
    }
}

#[async_trait]
impl MissionRepository for MissionRepositoryImpl {
    async fn start_mission(
        &self,
        tx: &DatabaseTransaction,
        user_id: Uuid,
        mission_id: i16,
    ) -> Result<(), DbErr> {
        missions_started::ActiveModel {
            user_id: Set(user_id),
            mission_id: Set(mission_id),
            mission_progress: Set(0),
        }
        .insert(tx)
        .await?;

        Ok(())
    }

    async fn progress_mission(
        &self,
        tx: &DatabaseTransaction,
        user_id: Uuid,
        mission_id: i16,
        new_progress: i32,
    ) -> Result<(), DbErr> {
        missions_started::Entity::update(missions_started::ActiveModel {
            user_id: Set(user_id),
            mission_id: Set(mission_id),
            mission_progress: Set(new_progress),
        })
        .exec(tx)
        .await?;

        Ok(())
    }

    async fn complete_mission(
        &self,
        tx: &DatabaseTransaction,
        user_id: Uuid,
        mission_id: i16,
    ) -> Result<(), DbErr> {
        Self::delete_started_mission(&self, tx, user_id, mission_id).await?;
        Self::add_completed_mission(&self, tx, user_id, mission_id).await?;
        Ok(())
    }
}
