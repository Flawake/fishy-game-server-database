use rocket::async_trait;
use sea_orm::{
    sea_query::{Expr, OnConflict}, ActiveValue::Set, ColumnTrait, Condition,
    DatabaseTransaction, DbErr, EntityTrait, ExprTrait, QueryFilter,
};
use uuid::Uuid;

use crate::{domain::{Durability, InventoryItem, Stack}, entity::{inventory_item, inventory_item_durability_codec, inventory_item_stack_codec}};

#[async_trait]
pub trait InventoryRepository: Send + Sync {
    /// Applies `item` to the inventory of `user_id`.
    ///
    /// The durability and stack of `item` are relative: they are added to the
    /// values that are already stored for the item, so a stack of `1` adds one
    /// to the stack and a stack of `-1` removes one from it. If the item does
    /// not exist yet, it is created with the given values.
    ///
    /// When the resulting durability or stack drops to zero or below, the item
    /// is removed from the inventory entirely.
    async fn add_or_update_item(
        &self,
        tx: &DatabaseTransaction,
        user_id: Uuid,
        items: Vec<InventoryItem>,
    ) -> Result<(), DbErr>;

    async fn destroy(
        &self,
        tx: &DatabaseTransaction,
        user_id: Uuid,
        item_uid: Uuid,
    ) -> Result<(), DbErr>;
}

#[derive(Debug, Clone)]
pub struct InventoryRepositoryImpl;

impl InventoryRepositoryImpl {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn add_or_update_item_definition(
        &self,
        tx: &DatabaseTransaction,
        user_id: Uuid,
        item_uuid: Uuid,
        definition_id: i32,
    ) -> Result<(), DbErr> {
        inventory_item::Entity::insert(inventory_item::ActiveModel {
            user_id: Set(user_id),
            item_uuid: Set(item_uuid),
            definition_id: Set(definition_id),
        })
        .on_conflict(OnConflict::new().do_nothing().to_owned())
        .exec_without_returning(tx)
        .await?;

        Ok(())
    }

    /// Adds `durability` to the durability that is already stored for the item
    /// and returns the resulting durability. Negative values subtract.
    pub async fn add_or_update_durability(
        &self,
        tx: &DatabaseTransaction,
        item_uuid: Uuid,
        durability: Durability,
    ) -> Result<i32, DbErr> {
        let model = inventory_item_durability_codec::Entity::insert(inventory_item_durability_codec::ActiveModel {
            item_uuid: Set(item_uuid),
            current_durability: Set(durability.durability),
        })
        .on_conflict(
            OnConflict::column(
                inventory_item_durability_codec::Column::ItemUuid
            )
            .value(
                inventory_item_durability_codec::Column::CurrentDurability,
                Expr::col((
                    inventory_item_durability_codec::Entity,
                    inventory_item_durability_codec::Column::CurrentDurability,
                ))
                .add(Expr::val(durability.durability)),
            )
            .to_owned(),
        )
        .exec_with_returning(tx)
        .await?;

        Ok(model.current_durability)
    }

    /// Adds `stack` to the stack that is already stored for the item and
    /// returns the resulting stack. Negative values subtract.
    pub async fn add_or_update_stack(
        &self,
        tx: &DatabaseTransaction,
        item_uuid: Uuid,
        stack: Stack,
    ) -> Result<i32, DbErr> {
        let model = inventory_item_stack_codec::Entity::insert(inventory_item_stack_codec::ActiveModel {
            item_uuid: Set(item_uuid),
            current_stack: Set(stack.stack),
        })
        .on_conflict(
            OnConflict::column(
                inventory_item_stack_codec::Column::ItemUuid
            )
            .value(
                inventory_item_stack_codec::Column::CurrentStack,
                Expr::col((
                    inventory_item_stack_codec::Entity,
                    inventory_item_stack_codec::Column::CurrentStack,
                ))
                .add(Expr::val(stack.stack)),
            )
            .to_owned(),
        )
        .exec_with_returning(tx)
        .await?;

        Ok(model.current_stack)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{DatabaseBackend, QueryTrait};

    /// The stack must be updated relative to the stored value, not overwritten.
    #[test]
    fn stack_upsert_is_relative() {
        let sql = inventory_item_stack_codec::Entity::insert(
            inventory_item_stack_codec::ActiveModel {
                item_uuid: Set(Uuid::new_v4()),
                current_stack: Set(-3),
            },
        )
        .on_conflict(
            OnConflict::column(inventory_item_stack_codec::Column::ItemUuid)
                .value(
                    inventory_item_stack_codec::Column::CurrentStack,
                    Expr::col((
                        inventory_item_stack_codec::Entity,
                        inventory_item_stack_codec::Column::CurrentStack,
                    ))
                    .add(Expr::val(-3)),
                )
                .to_owned(),
        )
        .build(DatabaseBackend::Postgres)
        .to_string();

        assert!(
            sql.contains(
                r#"ON CONFLICT ("item_uuid") DO UPDATE SET "current_stack" = "inventory_item_stack_codec"."current_stack" + -3"#
            ),
            "unexpected sql: {sql}"
        );
    }
}

#[async_trait]
impl InventoryRepository for InventoryRepositoryImpl {
    async fn add_or_update_item(
        &self,
        tx: &DatabaseTransaction,
        user_id: Uuid,
        items: Vec<InventoryItem>,
    ) -> Result<(), DbErr> {
        for item in items {
            self.add_or_update_item_definition(tx, user_id, item.item_uuid, item.definition_id).await?;
            let mut depleted = false;
            if let Some(durability) = item.durability.filter(|d| d.durability != 0) {
                depleted |= self.add_or_update_durability(tx, item.item_uuid, durability).await? <= 0;
            }
            if let Some(stack) = item.stack.filter(|s| s.stack != 0) {
                depleted |= self.add_or_update_stack(tx, item.item_uuid, stack).await? <= 0;
            }
        
            if depleted {
                self.destroy(tx, user_id, item.item_uuid).await?;
            }
        }

        Ok(())
    }

    async fn destroy(
        &self,
        tx: &DatabaseTransaction,
        user_id: Uuid,
        item_uid: Uuid,
    ) -> Result<(), DbErr> {
        let result = inventory_item::Entity::delete_many()
            .filter(
                Condition::all()
                    .add(inventory_item::Column::UserId.eq(user_id))
                    .add(inventory_item::Column::ItemUuid.eq(item_uid))
                    .to_owned(),
            )
            .exec(tx)
            .await?;

        if result.rows_affected == 0 {
            return Err(DbErr::RecordNotUpdated);
        }

        Ok(())
    }
}
