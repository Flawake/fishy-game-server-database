use std::collections::HashSet;

use chrono::{DateTime, Duration, TimeZone, Utc};
use rocket::async_trait;
use sea_orm::{DatabaseConnection, DbErr, TransactionError, TransactionTrait};
use uuid::Uuid;

use crate::{
    controller::herb_quest::HandInFish,
    fish_catalog,
    repository::{
        herb_quest::HerbQuestRepository, inventory::InventoryRepository, stats::StatsRepository,
    },
};

/// The current Herb quest, ready to be handed to the game.
pub struct CurrentQuest {
    pub quest_id: Uuid,
    pub area_id: i32,
    pub reward_coins: i32,
    pub next_move_at: DateTime<Utc>,
    /// (fish_id, amount) pairs the quest asks for.
    pub fishes: Vec<(i32, i32)>,
}

#[async_trait]
pub trait HerbQuestService: Send + Sync {
    /// Returns the currently active quest, generating one if none exists yet.
    async fn current_quest(&self) -> Result<CurrentQuest, DbErr>;

    /// Records that the player accepted (saw) the given quest.
    async fn accept_quest(&self, user_id: Uuid, quest_id: Uuid) -> Result<(), DbErr>;

    /// Hands in the quest fishes: validates against the stored quest, re-derives the
    /// reward from it (never trusting the client's value), applies the inventory changes,
    /// rewards the coins and records the completion, all atomically.
    /// Returns the awarded amount of coins.
    async fn complete_quest(
        &self,
        user_id: Uuid,
        quest_id: Uuid,
        fishes: Vec<HandInFish>,
    ) -> Result<i32, DbErr>;

    /// Ensures a valid current quest exists on boot, generating one when there is none
    /// or the stored one has already expired (e.g. the server was down across a rollover).
    async fn ensure_quest_on_boot(&self) -> Result<(), DbErr>;

    /// Generates and stores the next quest (Herb's next move). Called by the scheduler.
    async fn roll_over_quest(&self) -> Result<(), DbErr>;
}

pub struct HerbQuestServiceImpl<Q, I, S> {
    db: DatabaseConnection,
    herb_quest_repository: Q,
    inventory_repository: I,
    stats_repository: S,
}

impl<Q, I, S> HerbQuestServiceImpl<Q, I, S> {
    pub fn new(
        db: DatabaseConnection,
        herb_quest_repository: Q,
        inventory_repository: I,
        stats_repository: S,
    ) -> Self {
        Self {
            db,
            herb_quest_repository,
            inventory_repository,
            stats_repository,
        }
    }
}

/// The next 04:00 UTC strictly after `now`.
pub fn next_rollover(now: DateTime<Utc>) -> DateTime<Utc> {
    let today_four = now
        .date_naive()
        .and_hms_opt(4, 0, 0)
        .expect("04:00:00 is always a valid time");
    let today_four = Utc.from_utc_datetime(&today_four);

    if now < today_four {
        today_four
    } else {
        today_four + Duration::days(1)
    }
}

fn map_tx_err(err: TransactionError<DbErr>) -> DbErr {
    match err {
        TransactionError::Connection(e) => e,
        TransactionError::Transaction(e) => e,
    }
}

#[async_trait]
impl<
        Q: HerbQuestRepository + Clone + 'static,
        I: InventoryRepository + Clone + 'static,
        S: StatsRepository + Clone + 'static,
    > HerbQuestService for HerbQuestServiceImpl<Q, I, S>
{
    async fn current_quest(&self) -> Result<CurrentQuest, DbErr> {
        let repo = self.herb_quest_repository.clone();

        self.db
            .transaction::<_, CurrentQuest, DbErr>(move |tx| {
                Box::pin(async move {
                    let stored = match repo.get_current_quest(tx).await? {
                        Some(quest) => quest,
                        None => {
                            // Very first request before any quest exists: generate one so
                            // the endpoint always returns something valid.
                            let generated = fish_catalog::generate_quest();
                            let now = Utc::now();
                            repo.replace_current_quest(
                                tx,
                                Uuid::new_v4(),
                                generated.area_id,
                                generated.reward_coins,
                                next_rollover(now).fixed_offset(),
                                now.fixed_offset(),
                                generated.fishes,
                            )
                            .await?;
                            repo.get_current_quest(tx).await?.ok_or_else(|| {
                                DbErr::RecordNotFound(
                                    "herb quest missing right after generation".into(),
                                )
                            })?
                        }
                    };

                    let (quest, fishes) = stored;
                    Ok(CurrentQuest {
                        quest_id: quest.quest_id,
                        area_id: quest.area_id,
                        reward_coins: quest.reward_coins,
                        next_move_at: quest.next_move_at.with_timezone(&Utc),
                        fishes: fishes.into_iter().map(|f| (f.fish_id, f.amount)).collect(),
                    })
                })
            })
            .await
            .map_err(map_tx_err)
    }

    async fn accept_quest(&self, user_id: Uuid, quest_id: Uuid) -> Result<(), DbErr> {
        let repo = self.herb_quest_repository.clone();

        self.db
            .transaction::<_, (), DbErr>(move |tx| {
                Box::pin(async move {
                    // Only record acceptance for the quest that is actually current.
                    match repo.get_current_quest(tx).await? {
                        Some((quest, _)) if quest.quest_id == quest_id => {
                            repo.set_last_accepted(tx, user_id, quest_id).await
                        }
                        _ => Err(DbErr::Custom(
                            "Accepted quest is not the current Herb quest".into(),
                        )),
                    }
                })
            })
            .await
            .map_err(map_tx_err)
    }

    async fn complete_quest(
        &self,
        user_id: Uuid,
        quest_id: Uuid,
        fishes: Vec<HandInFish>,
    ) -> Result<i32, DbErr> {
        let quest_repo = self.herb_quest_repository.clone();
        let inventory_repo = self.inventory_repository.clone();
        let stats_repo = self.stats_repository.clone();

        self.db
            .transaction::<_, i32, DbErr>(move |tx| {
                Box::pin(async move {
                    let (quest, quest_fishes) = quest_repo
                        .get_current_quest(tx)
                        .await?
                        .ok_or_else(|| DbErr::Custom("No current Herb quest to complete".into()))?;

                    // The hand-in must target the quest that is currently active.
                    if quest.quest_id != quest_id {
                        return Err(DbErr::Custom(
                            "Handed-in quest is not the current Herb quest".into(),
                        ));
                    }

                    // Block a second hand-in of the same quest.
                    if let Some(state) = quest_repo.get_player_state(tx, user_id).await? {
                        if state.last_completed_quest_id == Some(quest_id) {
                            return Err(DbErr::Custom("Herb quest already completed".into()));
                        }
                    }

                    // Validate: every handed-in fish must be a species the quest asked for.
                    let allowed: HashSet<i32> =
                        quest_fishes.iter().map(|f| f.fish_id).collect();
                    for fish in &fishes {
                        if !allowed.contains(&fish.fish_id) {
                            return Err(DbErr::Custom(format!(
                                "Fish {} is not part of this Herb quest",
                                fish.fish_id
                            )));
                        }
                    }

                    // Apply the inventory changes the (trusted) game server computed.
                    for fish in fishes {
                        if fish.fish_amount <= 0 {
                            inventory_repo.destroy(tx, user_id, fish.fish_uid).await?;
                        } else {
                            let state_blob =
                                fish.new_state_blob.filter(|blob| !blob.is_empty()).ok_or_else(
                                    || {
                                        DbErr::Custom(
                                            "Missing state blob for a handed in fish stack".into(),
                                        )
                                    },
                                )?;
                            inventory_repo
                                .add_or_update_tx(
                                    tx,
                                    user_id,
                                    fish.fish_uid,
                                    fish.fish_id,
                                    state_blob,
                                )
                                .await?;
                        }
                    }

                    // Reward comes from the stored quest, never from the client-sent value.
                    let reward = quest.reward_coins;
                    stats_repo.change_coins_tx(tx, user_id, reward).await?;
                    quest_repo.set_last_completed(tx, user_id, quest_id).await?;

                    Ok(reward)
                })
            })
            .await
            .map_err(map_tx_err)
    }

    async fn ensure_quest_on_boot(&self) -> Result<(), DbErr> {
        let repo = self.herb_quest_repository.clone();

        let needs_new = self
            .db
            .transaction::<_, bool, DbErr>(move |tx| {
                Box::pin(async move {
                    match repo.get_current_quest(tx).await? {
                        None => Ok(true),
                        Some((quest, fishes)) => {
                            let expired = quest.next_move_at.with_timezone(&Utc) <= Utc::now();
                            Ok(expired || fishes.is_empty())
                        }
                    }
                })
            })
            .await
            .map_err(map_tx_err)?;

        if needs_new {
            self.roll_over_quest().await?;
        }
        Ok(())
    }

    async fn roll_over_quest(&self) -> Result<(), DbErr> {
        let repo = self.herb_quest_repository.clone();

        self.db
            .transaction::<_, (), DbErr>(move |tx| {
                Box::pin(async move {
                    let generated = fish_catalog::generate_quest();
                    let now = Utc::now();
                    repo.replace_current_quest(
                        tx,
                        Uuid::new_v4(),
                        generated.area_id,
                        generated.reward_coins,
                        next_rollover(now).fixed_offset(),
                        now.fixed_offset(),
                        generated.fishes,
                    )
                    .await
                })
            })
            .await
            .map_err(map_tx_err)
    }
}
