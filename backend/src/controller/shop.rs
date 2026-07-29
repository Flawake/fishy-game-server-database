use rocket::{post, routes, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{domain::InventoryItem, state::AppState};

/// Request body for buying an item.
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Copy)]
pub enum MoneyType {
    COINS,
    BUCKS,
}

/// Request body for buying an item.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct BuyItemRequest {
    pub buyer_id: Uuid,
    pub item: InventoryItem,
    pub item_price: i32,
    pub bought_using: MoneyType,
}

#[utoipa::path(
    post,
    path = "/shop/buy_item",
    request_body = BuyItemRequest,
    responses(
        (status = 200, description = "Item bough successfully", body = bool, content_type = "application/json"),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Buys an item",
    operation_id = "buy_item",
    tag = "Shop"
)]
#[post("/buy_item", data = "<payload>")]
async fn buy_item(
    payload: Json<BuyItemRequest>,
    state: &State<AppState>,
) -> Json<bool> {
    let inner = payload.into_inner();
    println!("{:?}", &inner);
    match state.shop
        .buy_item(
            inner.buyer_id,
            inner.item,
            inner.item_price,
            inner.bought_using,
        )
        .await
    {
        Ok(()) => Json(true),
        Err(e) => {
            println!("{}", e);
            Json(false)
        }
    }
}

pub fn shop_routes() -> Vec<rocket::Route> {
    routes![buy_item,]
}
