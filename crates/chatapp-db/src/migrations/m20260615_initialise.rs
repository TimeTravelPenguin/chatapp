use sea_orm_migration::prelude::*;

use crate::entity;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.get_schema_builder()
            .register(entity::users::Entity)
            .apply(db)
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(entity::users::Entity)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
