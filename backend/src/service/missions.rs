use crate::repository::{
    inventory::InventoryRepository, missions::MissionRepository, stats::StatsRepository,
};
use rocket::async_trait;
use sea_orm::{DatabaseConnection, DbErr, TransactionError, TransactionTrait};
use uuid::Uuid;

/// The single inventory row a mission reward writes.
pub struct MissionRewardItem {
    pub uuid: Uuid,
    pub definition_id: i32,
    pub state_blob: String,
}

/// Everything a mission pays out on completion.
pub struct MissionReward {
    pub coins: i32,
    pub bucks: i32,
    pub item: Option<MissionRewardItem>,
}

#[async_trait]
pub trait MissionService: Send + Sync {
    async fn start_mission(&self, user_id: Uuid, mission_id: i16) -> Result<(), DbErr>;

    async fn progress_mission(
        &self,
        user_id: Uuid,
        mission_id: i16,
        new_progress: i32,
    ) -> Result<(), DbErr>;

    /// Records the completion and pays the reward atomically.
    async fn complete_mission(
        &self,
        user_id: Uuid,
        mission_id: i16,
        reward: MissionReward,
    ) -> Result<(), DbErr>;
}

pub struct MissionServiceImpl<M, S, I> {
    db: DatabaseConnection,
    mission_repository: M,
    stats_repository: S,
    inventory_repository: I,
}

impl<M, S, I> MissionServiceImpl<M, S, I> {
    // create a new function for MissionServiceImpl.
    pub fn new(
        db: DatabaseConnection,
        mission_repository: M,
        stats_repository: S,
        inventory_repository: I,
    ) -> Self {
        Self {
            db,
            mission_repository,
            stats_repository,
            inventory_repository,
        }
    }
}

// Implement MissionService trait for MissionServiceImpl.
#[async_trait]
impl<
        R: MissionRepository + Clone + 'static,
        S: StatsRepository + Clone + 'static,
        I: InventoryRepository + Clone + 'static,
    > MissionService for MissionServiceImpl<R, S, I>
{
    async fn start_mission(&self, user_id: Uuid, mission_id: i16) -> Result<(), DbErr> {
        let mission_repo = self.mission_repository.clone();

        self.db
            .transaction::<_, (), DbErr>(move |tx| {
                Box::pin(async move { mission_repo.start_mission(tx, user_id, mission_id).await })
            })
            .await
            .map_err(|e| match e {
                TransactionError::Connection(e) => e,
                TransactionError::Transaction(e) => e,
            })
    }

    async fn progress_mission(
        &self,
        user_id: Uuid,
        mission_id: i16,
        new_progress: i32,
    ) -> Result<(), DbErr> {
        let mission_repo = self.mission_repository.clone();

        self.db
            .transaction::<_, (), DbErr>(move |tx| {
                Box::pin(async move {
                    mission_repo
                        .progress_mission(tx, user_id, mission_id, new_progress)
                        .await
                })
            })
            .await
            .map_err(|e| match e {
                TransactionError::Connection(e) => e,
                TransactionError::Transaction(e) => e,
            })
    }

    async fn complete_mission(
        &self,
        user_id: Uuid,
        mission_id: i16,
        reward: MissionReward,
    ) -> Result<(), DbErr> {
        let mission_repo = self.mission_repository.clone();
        let stats_repo = self.stats_repository.clone();
        let inventory_repo = self.inventory_repository.clone();

        self.db
            .transaction::<_, (), DbErr>(move |tx| {
                Box::pin(async move {
                    // Moves the started row into the completed table. The insert is
                    // deliberately strict: a second completion of the same mission
                    // hits the primary key and rolls the reward back with it.
                    mission_repo.complete_mission(tx, user_id, mission_id).await?;

                    if reward.coins != 0 {
                        stats_repo.change_coins_tx(tx, user_id, reward.coins).await?;
                    }

                    if reward.bucks != 0 {
                        stats_repo.change_bucks_tx(tx, user_id, reward.bucks).await?;
                    }

                    if let Some(item) = reward.item {
                        inventory_repo
                            .add_or_update_tx(
                                tx,
                                user_id,
                                item.uuid,
                                item.definition_id,
                                item.state_blob,
                            )
                            .await?;
                    }

                    Ok(())
                })
            })
            .await
            .map_err(|e| match e {
                TransactionError::Connection(e) => e,
                TransactionError::Transaction(e) => e,
            })
    }
}
