use std::path::PathBuf;

use chatapp_db::{
    UserActiveModel,
    migrations::{Migrator, MigratorTrait},
};
use sea_orm::{ActiveModelTrait, ActiveValue, Database, DatabaseConnection};
use thiserror::Error;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::hasher;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error(transparent)]
    Database(#[from] sea_orm::DbErr),
}

#[derive(Debug, Clone)]
pub struct DbStore {
    db: DatabaseConnection,
}

impl DbStore {
    pub async fn connect(database: PathBuf) -> Result<Self, StoreError> {
        let db =
            Database::connect(format!("sqlite://{}?mode=rwc", database.to_string_lossy())).await?;

        // TODO: better handle migrations at app startup.
        let pending = Migrator::get_pending_migrations(&db).await?;

        let store = Self { db };

        if !pending.is_empty() {
            Migrator::up(&store.db, None).await?;
            store.create_demo_user().await?; // TEMP
        }

        Ok(store)
    }

    pub async fn create_demo_user(&self) -> Result<(), StoreError> {
        let now = OffsetDateTime::now_utc();
        let hash = hasher::hash_password("penguins").expect("Failed to hash password");

        let user = UserActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            user_name: ActiveValue::Set("timetravelpenguin".to_string()),
            display_name: ActiveValue::Set("TimeTravelPenguin".to_string()),
            email: ActiveValue::Set("timetravelpenguin@gmail.com".to_string()),
            password_hash: ActiveValue::Set(hash.into_string()),
            created_at: ActiveValue::Set(now),
            updated_at: ActiveValue::Set(now),
        };

        user.insert(&self.db).await?;
        Ok(())
    }
}
