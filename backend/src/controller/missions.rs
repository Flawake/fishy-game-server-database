
use rocket::{post, routes, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

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
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct CompleteMissionRequest {
    pub user_id: Uuid,
    pub mission_id: i16,
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
        (status = 200, description = "Set successfully", body = bool, content_type = "application/json"),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "complete mission",
    operation_id = "complete_mission",
    tag = "Missions"
)]
#[post("/complete_mission", data = "<payload>")]
async fn complete_mission(
    payload: Json<CompleteMissionRequest>,
    state: &State<AppState>,
) -> Json<bool> {
    match state.mission
        .complete_mission(payload.user_id, payload.mission_id)
        .await
    {
        Ok(()) => Json(true),
        Err(_) => Json(false),
    }
}

// Combine all the data routes.
pub fn mission_routes() -> Vec<rocket::Route> {
    routes![start_mission, progress_mission, complete_mission]
}
