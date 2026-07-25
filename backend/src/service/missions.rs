use crate::repository::missions::MissionRepository;
use rocket::async_trait;
use sea_orm::{DatabaseConnection, DbErr, TransactionError, TransactionTrait};
use uuid::Uuid;

#[async_trait]
pub trait MissionService: Send + Sync {
    async fn start_mission(&self, user_id: Uuid, mission_id: i16) -> Result<(), DbErr>;

    async fn progress_mission(
        &self,
        user_id: Uuid,
        mission_id: i16,
        new_progress: i32,
    ) -> Result<(), DbErr>;

    async fn complete_mission(&self, user_id: Uuid, mission_id: i16) -> Result<(), DbErr>;
}

pub struct MissionServiceImpl<T: MissionRepository + Clone> {
    db: DatabaseConnection,
    mission_repository: T,
}

impl<R: MissionRepository + Clone> MissionServiceImpl<R> {
    // create a new function for MissionServiceImpl.
    pub fn new(db: DatabaseConnection, mission_repository: R) -> Self {
        Self {
            db,
            mission_repository,
        }
    }
}

// Implement MissionService trait for MissionServiceImpl.
#[async_trait]
impl<R: MissionRepository + Clone + 'static> MissionService for MissionServiceImpl<R> {
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

    async fn complete_mission(&self, user_id: Uuid, mission_id: i16) -> Result<(), DbErr> {
        let mission_repo = self.mission_repository.clone();

        self.db
            .transaction::<_, (), DbErr>(move |tx| {
                Box::pin(
                    async move { mission_repo.complete_mission(tx, user_id, mission_id).await },
                )
            })
            .await
            .map_err(|e| match e {
                TransactionError::Connection(e) => e,
                TransactionError::Transaction(e) => e,
            })
    }
}
