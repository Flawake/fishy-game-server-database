use std::sync::Arc;

use rocket::{post, routes, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::service::herb_quest::HerbQuestService;

/// A single fish stack the player hands in for the Herb quest.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HandInFish {
    pub fish_uid: Uuid,
    pub fish_id: i32,
    /// The amount left in the stack after handing in; 0 or less destroys the stack.
    pub fish_amount: i32,
    pub new_state_blob: Option<String>,
}

/// Request body for handing in the current Herb quest.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CompleteDailyQuestRequest {
    pub user_id: Uuid,
    pub herb_quest_id: Uuid,
    /// Sent by the game for reference; the database re-derives the reward from the
    /// stored quest and ignores this value.
    pub reward_coins: i32,
    pub fishes: Vec<HandInFish>,
}

/// Request body for recording that a player accepted (saw) the current Herb quest.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AcceptDailyQuestRequest {
    pub user_id: Uuid,
    pub herb_quest_id: Uuid,
}

/// One fish species the current Herb quest asks for.
#[derive(Debug, Serialize, ToSchema)]
pub struct HerbQuestFishDto {
    pub fish_id: i32,
    pub amount: i32,
}

/// The currently active Herb quest (global, same for every player).
#[derive(Debug, Serialize, ToSchema)]
pub struct CurrentHerbQuestResponse {
    pub herb_quest_id: Uuid,
    pub area_id: i32,
    pub reward_coins: i32,
    pub fishes: Vec<HerbQuestFishDto>,
    /// ISO 8601 / UTC, e.g. "2026-07-11T04:00:00+00:00".
    pub next_move_at: String,
}

#[utoipa::path(
    post,
    path = "/herb_quest/current_daily",
    responses(
        (status = 200, description = "The currently active Herb quest", body = CurrentHerbQuestResponse),
        (status = 500, description = "Internal server error")
    ),
    description = "Returns the Herb quest that is currently active. The database owns the quest; the game server polls this endpoint."
)]
#[post("/current_daily")]
pub async fn current_daily_quest(
    herb_quest_service: &State<Arc<dyn HerbQuestService>>,
) -> Option<Json<CurrentHerbQuestResponse>> {
    match herb_quest_service.current_quest().await {
        Ok(quest) => Some(Json(CurrentHerbQuestResponse {
            herb_quest_id: quest.quest_id,
            area_id: quest.area_id,
            reward_coins: quest.reward_coins,
            fishes: quest
                .fishes
                .into_iter()
                .map(|(fish_id, amount)| HerbQuestFishDto { fish_id, amount })
                .collect(),
            next_move_at: quest.next_move_at.to_rfc3339(),
        })),
        Err(e) => {
            eprintln!("Error fetching current Herb quest: {:?}", e);
            None
        }
    }
}

#[utoipa::path(
    post,
    path = "/herb_quest/complete_daily",
    request_body = CompleteDailyQuestRequest,
    responses(
        (status = 200, description = "Quest handed in, fishes removed and coins rewarded", body = bool),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Hand in the Herb quest fishes. Validates against the stored quest, re-derives the reward from it, and fails when the quest is stale or already completed."
)]
#[post("/complete_daily", data = "<payload>")]
pub async fn complete_daily_quest(
    payload: Json<CompleteDailyQuestRequest>,
    herb_quest_service: &State<Arc<dyn HerbQuestService>>,
) -> Json<bool> {
    let inner = payload.into_inner();
    match herb_quest_service
        .complete_quest(inner.user_id, inner.herb_quest_id, inner.fishes)
        .await
    {
        Ok(_reward) => Json(true),
        Err(e) => {
            eprintln!("Error completing Herb quest: {:?}", e);
            Json(false)
        }
    }
}

#[utoipa::path(
    post,
    path = "/herb_quest/accept_daily",
    request_body = AcceptDailyQuestRequest,
    responses(
        (status = 200, description = "Acceptance recorded", body = bool),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Records that a player accepted (saw) the current Herb quest so the game can skip Herb's introduction next time."
)]
#[post("/accept_daily", data = "<payload>")]
pub async fn accept_daily_quest(
    payload: Json<AcceptDailyQuestRequest>,
    herb_quest_service: &State<Arc<dyn HerbQuestService>>,
) -> Json<bool> {
    let inner = payload.into_inner();
    match herb_quest_service
        .accept_quest(inner.user_id, inner.herb_quest_id)
        .await
    {
        Ok(()) => Json(true),
        Err(e) => {
            eprintln!("Error accepting Herb quest: {:?}", e);
            Json(false)
        }
    }
}

pub fn herb_quest_routes() -> Vec<rocket::Route> {
    routes![current_daily_quest, complete_daily_quest, accept_daily_quest]
}
