
use rocket::{post, routes, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{domain::{Durability, InventoryItem, Stack}, state::AppState};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct AddOrUpdateItemRequest {
    pub user_id: Uuid,
    pub item_uuid: Uuid,
    pub definition_id: i32,
    pub durability: Option<Durability>,
    pub stack: Option<Stack>,
}

/// Request body for adding an item.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct DestroyItemRequest {
    pub user_id: Uuid,
    pub item_uid: Uuid,
}

// Utoipa is the crate that generates swagger documentation for your endpoints.
// The documentation for each endpoint is combined in docs.rs
// Make sure to add your endpoint in docs.rs when you write new endpoints.
#[utoipa::path(
    post,
    path = "/inventory/destroy",
    request_body = DestroyItemRequest,
    responses(
        (status = 200, description = "Item removed successfully", body = bool, content_type = "application/json"),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Removes an item from the database",
    operation_id = "destroy_item",
    tag = "Inventory"
)]
#[post("/destroy", data = "<payload>")]
async fn destroy_item(
    payload: Json<DestroyItemRequest>,
    state: &State<AppState>,
) -> Json<bool> {
    match state.inventory
        .destroy(payload.user_id, payload.item_uid)
        .await
    {
        Ok(()) => Json(true),
        Err(_) => Json(false),
    }
}

#[utoipa::path(
    post,
    path = "/inventory/addOrUpdate",
    request_body = AddOrUpdateItemRequest,
    responses(
        (status = 200, description = "Item added/updated successfully", body = bool, content_type = "application/json"),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Inserts an item in the database or updates it if it did already exist",
    operation_id = "add_or_update_item",
    tag = "Inventory"
)]
#[post("/add", data = "<payload>")]
async fn add_or_update_item(
    payload: Json<AddOrUpdateItemRequest>,
    state: &State<AppState>,
) -> Json<bool> {
    let inner = payload.into_inner();

    let item = InventoryItem {
                item_uuid: inner.item_uuid,
                definition_id: inner.definition_id,
                durability: inner.durability,
                stack: inner.stack,
            };
    
    match state.inventory
        .add_or_update_item(
            inner.user_id,
            item,
        )
        .await
    {
        Ok(()) => Json(true),
        Err(_) => Json(false),
    }
}

// Combine all the inventory routes.
pub fn inventory_routes() -> Vec<rocket::Route> {
    routes![destroy_item, add_or_update_item]
}
