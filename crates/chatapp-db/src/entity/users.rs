use sea_orm::entity::prelude::*;
use time::OffsetDateTime;
use uuid::Uuid;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(unique)]
    pub user_name: String,
    #[sea_orm(unique)]
    pub email: String,

    pub display_name: String,
    pub password_hash: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl ActiveModelBehavior for ActiveModel {}
