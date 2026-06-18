use std::path::PathBuf;

use chatapp_db::{
    UserActiveModel,
    migrations::{Migrator, MigratorTrait},
};
use sea_orm::{ActiveModelTrait, ActiveValue, Database, DatabaseConnection};
use thiserror::Error;

use crate::server::models::user::NewUser;

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

            // TEMP
            let user = NewUser::new(
                "TimeTravelPenguin",
                "Phillip Smith",
                "TimeTravelPenguin@gmail.com",
                "penguins",
            )
            .unwrap();
            store.create_user(user).await?;
        }

        Ok(store)
    }

    pub async fn create_user(&self, user: NewUser) -> Result<(), StoreError> {
        let user = UserActiveModel {
            id: ActiveValue::Set(user.id),
            username: ActiveValue::Set(user.username),
            display_name: ActiveValue::Set(user.display_name),
            email: ActiveValue::Set(user.email),
            password_hash: ActiveValue::Set(user.password_hash.to_string()),
            created_at: ActiveValue::Set(user.created_at),
            updated_at: ActiveValue::Set(user.created_at),
        };

        user.insert(&self.db).await?;
        Ok(())
    }
}
