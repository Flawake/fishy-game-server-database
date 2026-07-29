
use rocket::{post, routes, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{domain::InventoryItem, state::AppState};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct TradeRequest {
    pub user_one_id: Uuid,
    pub user_two_id: Uuid,
    pub user_one_receives: Vec<InventoryItem>,
    pub user_two_receives: Vec<InventoryItem>,
    pub user_one_bucks_received: i32,
    pub user_two_bucks_received: i32,
}

#[utoipa::path(
    post,
    path = "/trade/commit_trade",
    request_body = TradeRequest,
    responses(
        (status = 200, description = "trade items commited_successfully", body = bool, content_type = "application/json"),
        (status = 400, description = "invalid input data"),
        (status = 500, description = "Internal server error"),
    ),
    description = "Remove and add traded items to the accuonts",
    operation_id = "commit_trade",
    tag = "Trading"
)]
#[post("/commit_trade", data = "<payload>")]
async fn commit_trade(
    payload: Json<TradeRequest>,
    state: &State<AppState>,
) -> Json<bool> {
    let inner = payload.into_inner();
    match state.trade
        .commit_trade(
            inner.user_one_id,
            inner.user_two_id,
            inner.user_one_receives,
            inner.user_two_receives,
            inner.user_one_bucks_received,
            inner.user_two_bucks_received,
        )
        .await
    {
        Ok(()) => Json(true),
        Err(_) => Json(false),
    }
}

pub fn trade_routes() -> Vec<rocket::Route> {
    routes![commit_trade]
}
