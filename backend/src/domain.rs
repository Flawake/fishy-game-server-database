use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

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
