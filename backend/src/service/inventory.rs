use rocket::async_trait;
use uuid::Uuid;

use crate::repository::inventory::InventoryRepository;

// Here you add your business logic here.
#[async_trait]
pub trait InventoryService: Send + Sync {
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
        item_uid: Uuid,
    ) -> Result<(), sqlx::Error>;

    async fn degrade(
        &self,
        user_id: Uuid,
        item_uid: Uuid,
        amount: i32,
    ) -> Result<(), sqlx::Error>;
}

pub struct InventoryServiceImpl<T: InventoryRepository> {
    inventory_repository: T,
}

impl<R: InventoryRepository> InventoryServiceImpl<R> {
    // create a new function for InventoryServiceImpl.
    pub fn new(inventory_repository: R) -> Self {
        Self { inventory_repository }
    }
}

// Implement InventoryService trait for InventoryServiceImpl.
#[async_trait]
impl<R: InventoryRepository> InventoryService for InventoryServiceImpl<R> {
    async fn create(
        &self,
        user_id: Uuid,
        item_id: i32,
        item_uid: Uuid,
        amount: i32,
        cell_id: i32,
    ) -> Result<(), sqlx::Error> {
        if amount <= 0 {
            return Err(sqlx::Error::WorkerCrashed);
        }
        self.inventory_repository.create(user_id, item_id, item_uid, amount, cell_id).await
    }

    async fn increase(
        &self,
        user_id: Uuid,
        item_uid: Uuid,
        amount: i32,
    ) -> Result<(), sqlx::Error> {
        if amount <= 0 {
            return Err(sqlx::Error::WorkerCrashed);
        }
        self.inventory_repository.increase(user_id, item_uid, amount).await
    }

    async fn destroy(
        &self,
        user_id: Uuid,
        item_uid: Uuid,
    ) -> Result<(), sqlx::Error> {
        self.inventory_repository.destroy(user_id, item_uid).await
    }

    async fn degrade(
        &self,
        user_id: Uuid,
        item_uid: Uuid,
        amount: i32,
    ) -> Result<(), sqlx::Error> {
        if amount <= 0 {
            return Err(sqlx::Error::WorkerCrashed);
        }
        self.inventory_repository.degrade(user_id, item_uid, amount).await
    }
}
