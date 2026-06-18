use argon2::{
    Argon2, PasswordHasher, PasswordVerifier,
    password_hash::{self, PasswordHash, SaltString, rand_core::OsRng},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordHashError {
    #[error("password hash operation failed: {0}")]
    PasswordHash(#[from] password_hash::Error),
}

#[derive(Debug, Clone)]
pub struct PasswordHashString(String);

impl PasswordHashString {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl From<String> for PasswordHashString {
    fn from(value: String) -> Self {
        Self(value)
    }
}

pub fn hash_password(password: &str) -> Result<PasswordHashString, PasswordHashError> {
    let salt = SaltString::generate(&mut OsRng);

    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(PasswordHashString(hash))
}

pub fn verify_password(password: &str, stored_hash: &str) -> Result<bool, PasswordHashError> {
    let parsed_hash = PasswordHash::new(stored_hash)?;

    match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(password_hash::Error::Password) => Ok(false),
        Err(err) => Err(err.into()),
    }
}
