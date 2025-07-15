use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct LoginResponse {
    pub code: i16,
    pub jwt: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub salt: String,
    pub created: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, FromRow)]
pub struct StatFish {
    pub user_id: Uuid,
    pub fish_id: i32,
    pub length: i32,
    pub bait_id: i32,
    pub area_id: i32,
}

/// Request body for adding playtime of a player
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SelectItemRequest {
    pub user_id: Uuid,
    pub item_uid: Uuid,
    pub item_type: ItemType,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Copy)]
pub enum ItemType {
    Rod,
    Bait,
    Extra
}

// Struct to retreive user data
#[derive(Serialize, Debug, Deserialize)]
pub struct UserData {
    pub name: String,
    pub xp: i32,
    pub coins: i32,
    pub bucks: i32,
    pub total_playtime: i32,
    pub selected_rod: Option<Uuid>,
    pub selected_bait: Option<Uuid>,
    pub fish_data: Vec<FishData>,
    pub inventory_items: Vec<InventoryItem>,
    pub mailbox: Vec<MailEntry>,
    pub friends: Vec<Friend>,
    pub friend_requests: Vec<FriendRequest>
}

#[derive(Serialize, Debug, Deserialize)]
pub struct FishData {
    pub fish_id: i32,
    pub amount: i32,
    pub max_length: i32,
    pub first_caught: chrono::NaiveDate,
    pub areas: Vec<i32>,
    pub baits: Vec<i32>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct InventoryItem {
    pub item_id: i32,
    pub item_uid: Uuid,
    pub amount: i32,
    pub cell_id: i32,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct MailEntry {
    pub mail_id: Uuid,
    pub title: String,
    pub message: String,
    pub send_time: DateTime<Utc>,
    pub read: bool,
    pub archived: bool,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Friend {
    pub user_one: Uuid,
    pub user_two: Uuid,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct FriendRequest {
    pub user_one: Uuid,
    pub user_two: Uuid,
    pub request_sender_id: Uuid,
}
