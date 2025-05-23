use rocket::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

#[async_trait]
pub trait InventoryRepository: Send + Sync {
    async fn create(
        &self,
        user_id: Uuid,
        item_id: i32,
        item_uid: Uuid,
        amount: i32,
        cell_id: i32,
    ) -> Result<(), sqlx::Error>;

    async fn increase(
        &self,
        user_id: Uuid,
        item_uid: Uuid,
        amount: i32,
    ) -> Result<(), sqlx::Error>;

    async fn destroy(
        &self,
        user_id: Uuid,
        item_uid: Uuid
    ) -> Result<(), sqlx::Error>;

    async fn degrade(
        &self,
        user_id: Uuid,
        item_uid: Uuid,
        amount: i32,
    ) -> Result<(), sqlx::Error>;
}

#[derive(Debug, Clone)]
pub struct InventoryRepositoryImpl {
    pool: PgPool,
}

impl InventoryRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl InventoryRepository for InventoryRepositoryImpl {
    async fn create(
        &self,
        user_id: Uuid,
        item_id: i32,
        item_uid: Uuid,
        amount: i32,
        cell_id: i32,
    ) -> Result<(), sqlx::Error> {
        match sqlx::query!(
            "INSERT INTO inventory_item (user_id, item_id, item_uid, amount, cell_id)
            VALUES ($1, $2, $3, $4, $5)",
            user_id,
            item_id,
            item_uid,
            amount,
            cell_id,
        )
        .execute(&self.pool)
        .await {
            Ok(_) => Ok(()),
            Err(e) => {
                dbg!(&e);
                Err(e)
            }
        }
    }

    async fn increase(
        &self,
        user_id: Uuid,
        item_uid: Uuid,
        amount: i32,
    ) -> Result<(), sqlx::Error> {
        match sqlx::query!(
            "UPDATE inventory_item
            SET amount = amount + $1
            WHERE user_id = $2 AND item_uid = $3",
            amount,
            user_id,
            item_uid,
        )
        .fetch_optional(&self.pool)
        .await {
            Ok(o) => o,
            Err(e) => {
                dbg!(&e);
                return Err(e);
            }
        };
        Ok(())
    }

    async fn destroy(
        &self,
        user_id: Uuid,
        item_uid: Uuid,
    ) -> Result<(), sqlx::Error> {
        let result = match sqlx::query!(
            "DELETE FROM inventory_item WHERE
            user_id = $1 AND item_uid = $2",
            user_id,
            item_uid,
        )
        .execute(&self.pool)
        .await {
            Ok(o) => o,
            Err(e) => {
                dbg!(&e);
                return Err(e);
            }
        };

        if result.rows_affected() == 0 {
            return Err(Error::RowNotFound);
        }

        Ok(())
    }

    async fn degrade(
        &self,
        user_id: Uuid,
        item_uid: Uuid,
        amount: i32,
    ) -> Result<(), sqlx::Error> {
        match sqlx::query!(
            "UPDATE inventory_item
            SET amount = amount - $1
            WHERE user_id = $2 AND item_uid = $3",
            amount,
            user_id,
            item_uid,
        )
        .fetch_optional(&self.pool)
        .await {
            Ok(o) => o,
            Err(e) => {
                dbg!(&e);
                return Err(e);
            }
        };

        Ok(())
    }
}

