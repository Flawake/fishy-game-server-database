
use rocket::{post, routes, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::UserData;
use crate::state::AppState;

/// Request body for retrieving all data belonging to a single player.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct RetrieveDataRequest {
    pub user_id: Uuid,
}

// Utoipa is the crate that generates swagger documentation for your endpoints.
// The documentation for each endpoint is combined in docs.rs
// Make sure to add your endpoint in docs.rs when you write new endpoints.
#[utoipa::path(
    post,
    path = "/data/retrieve_all_playerdata",
    request_body = RetrieveDataRequest,
    responses(
        (status = 200, description = "Retrieved successfully. Null when the user has no data.", body = Option<UserData>, content_type = "application/json"),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Retrieves user data from the database",
    operation_id = "retrieve_all_playerdata",
    tag = "UserData"
)]
#[post("/retrieve_all_playerdata", data = "<payload>")]
async fn retrieve_player_data(
    payload: Json<RetrieveDataRequest>,
    state: &State<AppState>,
) -> Json<Option<UserData>> {
    match state.data.retrieve_all(payload.user_id).await {
        Ok(o) => Json(Some(o)),
        Err(_) => Json(None),
    }
}

// Combine all the data routes.
pub fn data_routes() -> Vec<rocket::Route> {
    routes![retrieve_player_data]
}
