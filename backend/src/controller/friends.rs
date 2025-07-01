use std::sync::Arc;

use rocket::{post, routes, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{service::friends::FriendService};

/// Request body for adding or removing friend.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct FriendRequests {
    pub user_one_id: Uuid,
    pub user_two_id: Uuid,
}

// Utoipa is the crate that generates swagger documentation for your endpoints.
// The documentation for each endpoint is combined in docs.rs
// Make sure to add your endpoint in docs.rs when you write new endpoints.
#[utoipa::path(
    post,
    path = "/friend/add_friend",
    request_body = FriendRequests,
    responses(
        (status = 201, description = "Successfully added friend", body = bool),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Adds a friend to the database",
    operation_id = "addFriend",
    tag = "friends"
)]
#[post("/add_friend", data = "<payload>")]
async fn add_friend(
    payload: Json<FriendRequests>,
    friends_service: &State<Arc<dyn FriendService>>,
) -> Json<bool> {
    match friends_service
        .add_friend(
            payload.user_one_id,
            payload.user_two_id,
        )
        .await
    {
        Ok(()) => Json(true),
        Err(_) => Json(false),
    }
}

#[utoipa::path(
    post,
    path = "/friend/remove_friend",
    request_body = FriendRequests,
    responses(
        (status = 201, description = "Successfully removed friend", body = bool),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Removes a friend from the database",
    operation_id = "removeFriend",
    tag = "friends"
)]
#[post("/remove_friend", data = "<payload>")]
async fn remove_friend(
    payload: Json<FriendRequests>,
    friends_service: &State<Arc<dyn FriendService>>,
) -> Json<bool> {
    match friends_service
        .remove_friend(
            payload.user_one_id,
            payload.user_two_id,
        )
        .await
    {
        Ok(()) => Json(true),
        Err(_) => Json(false),
    }
}

#[utoipa::path(
    post,
    path = "/friend/add_friend_request",
    request_body = FriendRequests,
    responses(
        (status = 201, description = "Successfully added a friend request", body = bool),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Adds a friend request to the database",
    operation_id = "addFriendRequest",
    tag = "friends"
)]
#[post("/add_friend_request", data = "<payload>")]
async fn add_friend_request(
    payload: Json<FriendRequests>,
    friends_service: &State<Arc<dyn FriendService>>,
) -> Json<bool> {
    match friends_service
        .add_friend_request(
            payload.user_one_id,
            payload.user_two_id,
        )
        .await
    {
        Ok(()) => Json(true),
        Err(_) => Json(false),
    }
}


#[utoipa::path(
    post,
    path = "/friend/remove_friend_request",
    request_body = FriendRequests,
    responses(
        (status = 201, description = "Successfully removed a friend request", body = bool),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Removes a friend request from the database",
    operation_id = "removeFriendRequest",
    tag = "friends"
)]
#[post("/remove_friend_request", data = "<payload>")]
async fn remove_friend_request(
    payload: Json<FriendRequests>,
    friends_service: &State<Arc<dyn FriendService>>,
) -> Json<bool> {
    match friends_service
        .remove_friend_request(
            payload.user_one_id,
            payload.user_two_id,
        )
        .await
    {
        Ok(()) => Json(true),
        Err(_) => Json(false),
    }
}

// Combine all the friend routes.
pub fn friend_routes() -> Vec<rocket::Route> {
    routes![add_friend, remove_friend, add_friend_request, remove_friend_request]
}
