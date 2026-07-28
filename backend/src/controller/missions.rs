
use rocket::{post, routes, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::service::missions::{MissionReward, MissionRewardItem};
use crate::state::AppState;

/// Request body for starting a new mission.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct StartMissionRequest {
    pub user_id: Uuid,
    pub mission_id: i16,
}

/// Request body for progressing a mission.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct ProgressMissionRequest {
    pub user_id: Uuid,
    pub mission_id: i16,
    pub new_progress: i32,
}

/// Request body for completing a mission.
///
/// The reward is applied in the same transaction as the completion itself, so a
/// player can never end up marked complete without being paid, or paid twice.
/// The game server owns the mission definitions, so the amounts are taken as
/// given rather than re-derived here.
///
/// The item fields are flat and always present rather than a nested optional:
/// the game serialises with Unity's JsonUtility, which writes a default-filled
/// object where a null would belong. A nil `reward_item_uuid` means the mission
/// rewards no item. Definition id 0 is a real item, so it cannot be the sentinel.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct CompleteMissionRequest {
    pub user_id: Uuid,
    pub mission_id: i16,
    pub reward_coins: i32,
    pub reward_bucks: i32,
    pub reward_item_definition_id: i32,
    pub reward_item_uuid: Uuid,
    /// Full state of the resulting stack, already merged by the game server.
    pub reward_item_state_blob: String,
}

// Utoipa is the crate that generates swagger documentation for your endpoints.
// The documentation for each endpoint is combined in docs.rs
// Make sure to add your endpoint in docs.rs when you write new endpoints.
#[utoipa::path(
    post,
    path = "/missions/start_mission",
    request_body = StartMissionRequest,
    responses(
        (status = 200, description = "Set successfully", body = bool, content_type = "application/json"),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Starts a mission for a player",
    operation_id = "start_mission",
    tag = "Missions"
)]
#[post("/start_mission", data = "<payload>")]
async fn start_mission(
    payload: Json<StartMissionRequest>,
    state: &State<AppState>,
) -> Json<bool> {
    match state.mission
        .start_mission(payload.user_id, payload.mission_id)
        .await
    {
        Ok(()) => Json(true),
        Err(_) => Json(false),
    }
}

// Utoipa is the crate that generates swagger documentation for your endpoints.
// The documentation for each endpoint is combined in docs.rs
// Make sure to add your endpoint in docs.rs when you write new endpoints.
#[utoipa::path(
    post,
    path = "/missions/progress_mission",
    request_body = ProgressMissionRequest,
    responses(
        (status = 200, description = "Set successfully", body = bool, content_type = "application/json"),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Sets new mission progress",
    operation_id = "progress_mission",
    tag = "Missions"
)]
#[post("/progress_mission", data = "<payload>")]
async fn progress_mission(
    payload: Json<ProgressMissionRequest>,
    state: &State<AppState>,
) -> Json<bool> {
    match state.mission
        .progress_mission(payload.user_id, payload.mission_id, payload.new_progress)
        .await
    {
        Ok(()) => Json(true),
        Err(_) => Json(false),
    }
}

// Utoipa is the crate that generates swagger documentation for your endpoints.
// The documentation for each endpoint is combined in docs.rs
// Make sure to add your endpoint in docs.rs when you write new endpoints.
#[utoipa::path(
    post,
    path = "/missions/complete_mission",
    request_body = CompleteMissionRequest,
    responses(
        (status = 200, description = "Mission completed and reward paid", body = bool, content_type = "application/json"),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Completes a mission and pays out its reward in a single transaction.",
    operation_id = "complete_mission",
    tag = "Missions"
)]
#[post("/complete_mission", data = "<payload>")]
async fn complete_mission(
    payload: Json<CompleteMissionRequest>,
    state: &State<AppState>,
) -> Json<bool> {
    let inner = payload.into_inner();

    let item = if inner.reward_item_uuid.is_nil() {
        None
    } else {
        Some(MissionRewardItem {
            uuid: inner.reward_item_uuid,
            definition_id: inner.reward_item_definition_id,
            state_blob: inner.reward_item_state_blob,
        })
    };

    let reward = MissionReward {
        coins: inner.reward_coins,
        bucks: inner.reward_bucks,
        item,
    };

    match state.mission
        .complete_mission(inner.user_id, inner.mission_id, reward)
        .await
    {
        Ok(()) => Json(true),
        Err(e) => {
            eprintln!("Error completing mission: {:?}", e);
            Json(false)
        }
    }
}

// Combine all the data routes.
pub fn mission_routes() -> Vec<rocket::Route> {
    routes![start_mission, progress_mission, complete_mission]
}
