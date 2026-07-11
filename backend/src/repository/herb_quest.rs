use rocket::async_trait;
use sea_orm::{
    prelude::DateTimeWithTimeZone, sea_query::OnConflict, ActiveValue::Set, ColumnTrait,
    DatabaseTransaction, DbErr, EntityTrait, QueryFilter,
};
use uuid::Uuid;

use crate::entity::{herb_quest, herb_quest_fish, herb_quest_player_state};

#[async_trait]
pub trait HerbQuestRepository: Send + Sync {
    /// Reads the single, globally shared current Herb quest together with its fishes.
    async fn get_current_quest(
        &self,
        tx: &DatabaseTransaction,
    ) -> Result<Option<(herb_quest::Model, Vec<herb_quest_fish::Model>)>, DbErr>;

    /// Replaces the current quest (and its fishes) with a freshly generated one.
    /// Only the current quest is ever kept, so the previous rows are removed first.
    async fn replace_current_quest(
        &self,
        tx: &DatabaseTransaction,
        quest_id: Uuid,
        area_id: i32,
        reward_coins: i32,
        next_move_at: DateTimeWithTimeZone,
        created_at: DateTimeWithTimeZone,
        fishes: Vec<(i32, i32)>,
    ) -> Result<(), DbErr>;

    async fn get_player_state(
        &self,
        tx: &DatabaseTransaction,
        user_id: Uuid,
    ) -> Result<Option<herb_quest_player_state::Model>, DbErr>;

    /// Records that the player accepted (saw) the given quest.
    async fn set_last_accepted(
        &self,
        tx: &DatabaseTransaction,
        user_id: Uuid,
        quest_id: Uuid,
    ) -> Result<(), DbErr>;

    /// Records that the player completed the given quest.
    async fn set_last_completed(
        &self,
        tx: &DatabaseTransaction,
        user_id: Uuid,
        quest_id: Uuid,
    ) -> Result<(), DbErr>;
}

#[derive(Debug, Clone)]
pub struct HerbQuestRepositoryImpl;

impl HerbQuestRepositoryImpl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl HerbQuestRepository for HerbQuestRepositoryImpl {
    async fn get_current_quest(
        &self,
        tx: &DatabaseTransaction,
    ) -> Result<Option<(herb_quest::Model, Vec<herb_quest_fish::Model>)>, DbErr> {
        let quest = herb_quest::Entity::find().one(tx).await?;

        match quest {
            None => Ok(None),
            Some(quest) => {
                let fishes = herb_quest_fish::Entity::find()
                    .filter(herb_quest_fish::Column::QuestId.eq(quest.quest_id))
                    .all(tx)
                    .await?;
                Ok(Some((quest, fishes)))
            }
        }
    }

    async fn replace_current_quest(
        &self,
        tx: &DatabaseTransaction,
        quest_id: Uuid,
        area_id: i32,
        reward_coins: i32,
        next_move_at: DateTimeWithTimeZone,
        created_at: DateTimeWithTimeZone,
        fishes: Vec<(i32, i32)>,
    ) -> Result<(), DbErr> {
        // Drop the previous quest; the fish rows cascade away with it.
        herb_quest::Entity::delete_many().exec(tx).await?;

        herb_quest::Entity::insert(herb_quest::ActiveModel {
            quest_id: Set(quest_id),
            area_id: Set(area_id),
            reward_coins: Set(reward_coins),
            next_move_at: Set(next_move_at),
            created_at: Set(created_at),
            ..Default::default()
        })
        .exec_without_returning(tx)
        .await?;

        if !fishes.is_empty() {
            let fish_models = fishes.into_iter().map(|(fish_id, amount)| {
                herb_quest_fish::ActiveModel {
                    quest_id: Set(quest_id),
                    fish_id: Set(fish_id),
                    amount: Set(amount),
                    ..Default::default()
                }
            });
            herb_quest_fish::Entity::insert_many(fish_models)
                .exec_without_returning(tx)
                .await?;
        }

        Ok(())
    }

    async fn get_player_state(
        &self,
        tx: &DatabaseTransaction,
        user_id: Uuid,
    ) -> Result<Option<herb_quest_player_state::Model>, DbErr> {
        herb_quest_player_state::Entity::find_by_id(user_id)
            .one(tx)
            .await
    }

    async fn set_last_accepted(
        &self,
        tx: &DatabaseTransaction,
        user_id: Uuid,
        quest_id: Uuid,
    ) -> Result<(), DbErr> {
        herb_quest_player_state::Entity::insert(herb_quest_player_state::ActiveModel {
            user_id: Set(user_id),
            last_accepted_quest_id: Set(Some(quest_id)),
            last_completed_quest_id: Set(None),
            ..Default::default()
        })
        .on_conflict(
            OnConflict::column(herb_quest_player_state::Column::UserId)
                .update_column(herb_quest_player_state::Column::LastAcceptedQuestId)
                .to_owned(),
        )
        .exec_without_returning(tx)
        .await?;

        Ok(())
    }

    async fn set_last_completed(
        &self,
        tx: &DatabaseTransaction,
        user_id: Uuid,
        quest_id: Uuid,
    ) -> Result<(), DbErr> {
        herb_quest_player_state::Entity::insert(herb_quest_player_state::ActiveModel {
            user_id: Set(user_id),
            last_completed_quest_id: Set(Some(quest_id)),
            last_accepted_quest_id: Set(None),
            ..Default::default()
        })
        .on_conflict(
            OnConflict::column(herb_quest_player_state::Column::UserId)
                .update_column(herb_quest_player_state::Column::LastCompletedQuestId)
                .to_owned(),
        )
        .exec_without_returning(tx)
        .await?;

        Ok(())
    }
}
