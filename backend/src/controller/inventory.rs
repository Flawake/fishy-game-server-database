use std::sync::Arc;

use rocket::{post, routes, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::service::inventory::InventoryService;

/// Request body for adding an item.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct AddItemRequest {
    pub user_id: Uuid,
    pub item_id: i32,
    pub item_uid: Option<Uuid>,
    pub amount: i32,
    pub cell_id: i32,
}

/// Request body for adding an item.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct DegradeItemRequest {
    pub user_id: Uuid,
    pub item_id: i32,
    pub item_uid: Option<Uuid>,
    pub amount: i32,
}

/// Request body for adding an item.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct IncreaseItemRequest {
    pub user_id: Uuid,
    pub item_id: i32,
    pub item_uid: Option<Uuid>,
    pub amount: i32,
}

/// Request body for adding an item.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct DestroyItemRequest {
    pub user_id: Uuid,
    pub item_id: i32,
    pub item_uid: Option<Uuid>,
}


// Utoipa is the crate that generates swagger documentation for your endpoints.
// The documentation for each endpoint is combined in docs.rs
// Make sure to add your endpoint in docs.rs when you write new endpoints.
#[utoipa::path(
    post,
    path = "/inventory/add",
    request_body = AddItemRequest,
    responses(
        (status = 201, description = "Item added successfully", body = bool),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Adds an item to the database",
    operation_id = "addItem",
    tag = "Inventory"
)]
#[post("/add", data = "<payload>")]
async fn add_item(
    payload: Json<AddItemRequest>,
    inventory_service: &State<Arc<dyn InventoryService>>,
) -> Json<bool> {
    match inventory_service
        .create(
            payload.user_id,
            payload.item_id,
            payload.item_uid,
            payload.amount,
            payload.cell_id,
        )
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
    path = "/inventory/increase",
    request_body = IncreaseItemRequest,
    responses(
        (status = 201, description = "Item amount increased successfully", body = bool),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Increases the amount of an item in the database",
    operation_id = "increaseItem",
    tag = "Inventory"
)]
#[post("/increase", data = "<payload>")]
async fn increase_item(
    payload: Json<IncreaseItemRequest>,
    inventory_service: &State<Arc<dyn InventoryService>>,
) -> Json<bool> {
    match inventory_service
        .increase(
            payload.user_id,
            payload.item_id,
            payload.item_uid,
            payload.amount,
        )
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
    path = "/inventory/degrade",
    request_body = DegradeItemRequest,
    responses(
        (status = 201, description = "Item degraded successfully", body = bool),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Degrades an item in the database",
    operation_id = "degradeItem",
    tag = "Inventory"
)]
#[post("/degrade", data = "<payload>")]
async fn degrade_item(
    payload: Json<DegradeItemRequest>,
    inventory_service: &State<Arc<dyn InventoryService>>,
) -> Json<bool> {
    match inventory_service
        .degrade(
            payload.user_id,
            payload.item_id,
            payload.item_uid,
            payload.amount,
        )
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
    path = "/inventory/destroy",
    request_body = DestroyItemRequest,
    responses(
        (status = 201, description = "Item removed successfully", body = bool),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Removes an item from the database",
    operation_id = "destroyItem",
    tag = "Inventory"
)]
#[post("/destroy", data = "<payload>")]
async fn destroy_item(
    payload: Json<DestroyItemRequest>,
    inventory_service: &State<Arc<dyn InventoryService>>,
) -> Json<bool> {
    match inventory_service
        .destroy(
            payload.user_id,
            payload.item_id,
            payload.item_uid,
        )
        .await
    {
        Ok(()) => Json(true),
        Err(_) => Json(false),
    }
}

// Combine all the inventory routes.
pub fn inventory_routes() -> Vec<rocket::Route> {
    routes![add_item, increase_item, degrade_item, destroy_item]
}
