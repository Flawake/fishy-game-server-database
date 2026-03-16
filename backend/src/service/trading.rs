use rocket::{
    async_trait,
    futures::{stream::FuturesUnordered, StreamExt},
};
use sea_orm::{DatabaseConnection, DbErr, TransactionError, TransactionTrait};
use uuid::Uuid;

use crate::{
    controller::trading::TradeItemRequest,
    repository::{inventory::InventoryRepository, stats::StatsRepository},
};

#[async_trait]
pub trait TradeService: Send + Sync {
    async fn commit_trade(
        &self,
        user_one_id: Uuid,
        user_two_id: Uuid,
        user_one_receives: Vec<TradeItemRequest>,
        user_two_receives: Vec<TradeItemRequest>,
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
        user_one_receives: Vec<TradeItemRequest>,
        user_two_receives: Vec<TradeItemRequest>,
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

                    for item in user_one_receives {
                        if item.item_amount <= 0 {
                            tasks.push(inventory_repo.destroy(tx, user_one_id, item.item_uid));
                        } else {
                            tasks.push(inventory_repo.add_or_update_tx(
                                tx,
                                user_one_id,
                                item.item_uid,
                                item.item_id,
                                item.state_blob,
                            ));
                        }
                    }

                    for item in user_two_receives {
                        if item.item_amount <= 0 {
                            tasks.push(inventory_repo.destroy(tx, user_two_id, item.item_uid));
                        } else {
                            tasks.push(inventory_repo.add_or_update_tx(
                                tx,
                                user_two_id,
                                item.item_uid,
                                item.item_id,
                                item.state_blob,
                            ));
                        }
                    }

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
