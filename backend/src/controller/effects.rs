use crate::domain::AddActiveEffectRequest;
use rocket::{post, routes, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use crate::state::AppState;

/// Request body for removing expired effects for a specific user
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RemoveExpiredEffectRequest {
    pub user_id: Uuid,
    pub item_id: i32,
}

#[utoipa::path(
    post,
    path = "/effects/add_effect",
    request_body = AddActiveEffectRequest,
    responses(
        (status = 200, description = "Effect added successfully", body = bool, content_type = "application/json"),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    ),
    operation_id = "add_effect",
    tag = "Effects"
)]
#[post("/add_effect", data = "<add_request>")]
pub async fn add_effect(
    add_request: Json<AddActiveEffectRequest>,
    state: &State<AppState>,
) -> Json<bool> {
    match state.effects.add_effect(add_request.into_inner()).await {
        Ok(_) => Json(true),
        Err(e) => {
            eprintln!("Error adding effect: {:?}", e);
            Json(false)
        }
    }
}

#[utoipa::path(
    post,
    path = "/effects/remove_expired",
    request_body = RemoveExpiredEffectRequest,
    responses(
        (status = 200, description = "Expired effects removed", body = bool, content_type = "application/json"),
        (status = 500, description = "Internal server error")
    ),
    operation_id = "remove_expired_effects",
    tag = "Effects"
)]
#[post("/remove_expired", data = "<request>")]
pub async fn remove_expired_effects(
    request: Json<RemoveExpiredEffectRequest>,
    state: &State<AppState>,
) -> Json<bool> {
    match state.effects
        .remove_effect(request.user_id, request.item_id)
        .await
    {
        Ok(_) => Json(true),
        Err(e) => {
            eprintln!("Error removing expired effects: {:?}", e);
            Json(false)
        }
    }
}

#[utoipa::path(
    post,
    path = "/effects/cleanup_all_expired",
    responses(
        (status = 200, description = "All expired effects cleaned up", body = bool, content_type = "application/json"),
        (status = 500, description = "Internal server error")
    ),
    operation_id = "cleanup_all_expired_effects",
    tag = "Effects"
)]
#[post("/cleanup_all_expired")]
pub async fn cleanup_all_expired_effects(
    state: &State<AppState>,
) -> Json<bool> {
    match state.effects.cleanup_all_expired_effects().await {
        Ok(_) => Json(true),
        Err(e) => {
            eprintln!("Error cleaning up all expired effects: {:?}", e);
            Json(false)
        }
    }
}

pub fn effect_routes() -> Vec<rocket::Route> {
    routes![
        add_effect,
        remove_expired_effects,
        cleanup_all_expired_effects
    ]
}
