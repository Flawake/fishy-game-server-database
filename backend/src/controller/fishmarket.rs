
use rocket::{post, routes, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{domain::InventoryItem, state::AppState};

/// Request body for selling a fish
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SellFishesRequest {
    pub seller_id: Uuid,
    pub fishes: Vec<InventoryItem>,
    pub price: i32,
}

#[utoipa::path(
    post,
    path = "/fish_market/sell_fishes",
    request_body = SellFishesRequest,
    responses(
        (status = 200, description = "Fishes sold successfully", body = bool, content_type = "application/json"),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    ),
    operation_id = "sell_fishes",
    tag = "FishMarket"
)]
#[post("/sell_fishes", data = "<payload>")]
pub async fn sell_fishes(
    payload: Json<SellFishesRequest>,
    state: &State<AppState>,
) -> Json<bool> {
    let inner = payload.into_inner();
    match state.fishmarket
        .sell_fishes(inner.seller_id, inner.fishes, inner.price)
        .await
    {
        Ok(_) => Json(true),
        Err(e) => {
            eprintln!("Error selling fishes: {:?}", e);
            Json(false)
        }
    }
}

pub fn fishmarket_routes() -> Vec<rocket::Route> {
    routes![sell_fishes,]
}
