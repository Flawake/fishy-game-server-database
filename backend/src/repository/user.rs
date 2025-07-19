use crate::domain::User;
use rocket::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::types::Uuid;
use sqlx::PgPool;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> Result<(), sqlx::Error>;

    async fn from_uuid(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error>;

    async fn get_username_from_email(&self, email: String) -> Result<Option<Username>, sqlx::Error>;

    async fn from_username(&self, email: String) -> Result<Option<User>, sqlx::Error>;

    // add more functions such as update or delete.
}

#[derive(Debug, Clone)]
pub struct UserRepositoryImpl {
    pool: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, FromRow)]
pub struct Username {
    pub name: String,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, user: User) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // Insert user
        if let Err(e) = sqlx::query!(
            "INSERT INTO users (user_id, name, email, password, salt, created)
             VALUES ($1, $2, $3, $4, $5, $6)",
            user.user_id,
            user.name,
            user.email,
            user.password,
            user.salt,
            user.created
        )
        .execute(&mut *tx)
        .await {
            eprintln!("Error inserting user into database: {:?}", e);
            return Err(e);
        }

        // Insert stats
        let result = match sqlx::query!(
            "INSERT INTO stats (user_id, xp, coins, bucks, total_playtime)
            VALUES ($1, $2, $3, $4, $5);",
            user.user_id,
            0,    // xp
            25,   // coins
            5000, // bucks
            0     // total_playtime
        )
        .execute(&mut *tx)
        .await {
            Ok(o) => o,
            Err(e) => {
                dbg!(&e);
                return Err(e);
            }
        };

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        // Insert bamboo rod
        let result = match sqlx::query!(
            "INSERT INTO inventory_item (user_id, item_uuid, definition_id, state_blob)
            VALUES ($1, $2, $3, $4);",
            user.user_id,     // user_id
            Uuid::new_v4(),   // item_uid
            1000,             // definition_id
            "",               // state_blob
        )
        .execute(&mut *tx)
        .await {
            Ok(o) => o,
            Err(e) => {
                dbg!(&e);
                return Err(e);
            }
        };

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        // Insert hook
        let result = match sqlx::query!(
            "INSERT INTO inventory_item (user_id, item_uuid, definition_id, state_blob)
            VALUES ($1, $2, $3, $4);",
            user.user_id,     // user_id
            Uuid::new_v4(),   // item_uid
            0,                // definition_id
            "",               // state_blob
        )
        .execute(&mut *tx)
        .await {
            Ok(o) => o,
            Err(e) => {
                dbg!(&e);
                return Err(e);
            }
        };

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        if let Err(e) = tx.commit().await {
            dbg!(&e);
            return Err(e);
        };
        Ok(())
    }

    async fn from_uuid(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT user_id, name, email, password, salt, created
             FROM users
             WHERE user_id = $1",
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get_username_from_email(&self, email: String) -> Result<Option<Username>, sqlx::Error> {
        let user = match sqlx::query_as!(
            Username,
            "SELECT name
             FROM users
             WHERE email = $1",
            email
        )
        .fetch_optional(&self.pool)
        .await {
            Ok(o) => o,
            Err(e) => {
                dbg!(&e);
                return Err(e);
            }
        };
        Ok(user)
    }

    async fn from_username(&self, email: String) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT user_id, name, email, password, salt, created
             FROM users
             WHERE name = $1",
            email
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }
}
