use rocket::{
    async_trait, futures::{StreamExt, stream::FuturesUnordered},
};
use sea_orm::{DatabaseConnection, DbErr, TransactionError, TransactionTrait};
use uuid::Uuid;

use crate::{
    domain::InventoryItem, repository::{inventory::InventoryRepository, stats::StatsRepository},
};

#[async_trait]
pub trait TradeService: Send + Sync {
    async fn commit_trade(
        &self,
        user_one_id: Uuid,
        user_two_id: Uuid,
        user_one_receives: Vec<InventoryItem>,
        user_two_receives: Vec<InventoryItem>,
        user_one_bucks_received: i32,
        user_two_bucks_received: i32,
    ) -> Result<(), DbErr>;
}

pub struct TradeServiceImpl<I: InventoryRepository, S: StatsRepository> {
    db: DatabaseConnection,
    inventory_repository: I,
    stats_repository: S,
}

impl<I: InventoryRepository, S: StatsRepository> TradeServiceImpl<I, S> {
    // create a new function for TradeServiceImpl.
    pub fn new(db: DatabaseConnection, inventory_repository: I, stats_repository: S) -> Self {
        Self {
            db,
            inventory_repository,
            stats_repository,
        }
    }
}

// Implement TradeService trait for TradeServiceImpl.
#[async_trait]
impl<I: InventoryRepository + Clone + 'static, S: StatsRepository + Clone + 'static> TradeService
    for TradeServiceImpl<I, S>
{
    async fn commit_trade(
        &self,
        user_one_id: Uuid,
        user_two_id: Uuid,
        user_one_receives: Vec<InventoryItem>,
        user_two_receives: Vec<InventoryItem>,
        user_one_bucks_received: i32,
        user_two_bucks_received: i32,
    ) -> Result<(), DbErr> {
        let inventory_repo = self.inventory_repository.clone();
        let stats_repo = self.stats_repository.clone();
        self.db
            .transaction::<_, (), DbErr>(move |tx| {
                Box::pin(async move {
                    let delta_bucks_user_one = user_one_bucks_received - user_two_bucks_received;
                    let delta_bucks_user_two = user_two_bucks_received - user_one_bucks_received;
                    if delta_bucks_user_one != 0 {
                        stats_repo
                            .change_bucks_tx(tx, user_one_id, delta_bucks_user_one)
                            .await?;
                    }
                    if delta_bucks_user_two != 0 {
                        stats_repo
                            .change_bucks_tx(tx, user_two_id, delta_bucks_user_two)
                            .await?;
                    }

                    let mut tasks = FuturesUnordered::new();

                    tasks.push(inventory_repo.add_or_update_item(
                        tx,
                        user_one_id,
                        user_one_receives,
                    ));

                    tasks.push(inventory_repo.add_or_update_item(
                        tx,
                        user_two_id,
                        user_two_receives,
                    ));

                    while let Some(_) = tasks.next().await {}
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
