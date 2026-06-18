use argon2::password_hash::PasswordHashString;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::server::password::{PasswordHashError, hash_password};

#[derive(Debug, Clone)]
pub struct NewUser {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password_hash: PasswordHashString,
    pub created_at: OffsetDateTime,
}

impl NewUser {
    pub fn new(
        username: impl Into<String>,
        display_name: impl Into<String>,
        email: impl Into<String>,
        password: impl AsRef<str>,
    ) -> Result<Self, PasswordHashError> {
        Ok(Self {
            id: Uuid::new_v4(),
            username: username.into(),
            display_name: display_name.into(),
            email: email.into().to_ascii_lowercase(),
            password_hash: hash_password(password.as_ref())?,
            created_at: OffsetDateTime::now_utc(),
        })
    }
}
