use rocket::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

#[async_trait]
pub trait StatsRepository: Send + Sync {
    async fn add_xp(&self, user_id: Uuid, amount: i32) -> Result<(), sqlx::Error>;

    async fn change_bucks(&self, user_id: Uuid, amount: i32) -> Result<(), sqlx::Error>;

    async fn change_coins(&self, user_id: Uuid, amount: i32) -> Result<(), sqlx::Error>;

    async fn add_playtime(&self, user_id: Uuid, amount: i32) -> Result<(), sqlx::Error>;
}

#[derive(Debug, Clone)]
pub struct StatsRepositoryImpl {
    pool: PgPool,
}

impl StatsRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StatsRepository for StatsRepositoryImpl {
    async fn add_xp(&self, user_id: Uuid, amount: i32) -> Result<(), sqlx::Error> {
        let result = sqlx::query!(
            "UPDATE stats
            SET xp = xp + $1
            WHERE user_id = $2",
            amount,
            user_id,
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(Error::RowNotFound);
        }

        Ok(())
    }

    async fn change_bucks(&self, user_id: Uuid, amount: i32) -> Result<(), sqlx::Error> {
        let result = sqlx::query!(
            "UPDATE stats
            SET bucks = bucks + $1
            WHERE user_id = $2",
            amount,
            user_id,
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(Error::RowNotFound);
        }

        Ok(())
    }

    async fn change_coins(&self, user_id: Uuid, amount: i32) -> Result<(), sqlx::Error> {
        let result = sqlx::query!(
            "UPDATE stats
            SET coins = coins + $1
            WHERE user_id = $2",
            amount,
            user_id,
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(Error::RowNotFound);
        }

        Ok(())
    }

    async fn add_playtime(&self, user_id: Uuid, amount: i32) -> Result<(), sqlx::Error> {
        let result = sqlx::query!(
            "UPDATE stats
            SET total_play_time = total_play_time + $1
            WHERE user_id = $2",
            amount,
            user_id,
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(Error::RowNotFound);
        }

        Ok(())
    }
}
