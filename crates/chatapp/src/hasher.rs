use std::fmt::Display;

use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{self, SaltString, rand_core::OsRng},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HashError {
    #[error("Password hashing failed: {0}")]
    Hashing(#[from] password_hash::Error),
}

#[derive(Debug, Clone)]
pub struct HashedPassword(pub String);

impl From<String> for HashedPassword {
    fn from(s: String) -> Self {
        HashedPassword(s)
    }
}

impl Display for HashedPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn hash_password(password: &str) -> Result<HashedPassword, HashError> {
    let salt = SaltString::generate(&mut OsRng);

    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string()
        .into();

    Ok(hash)
}

pub fn verify_password(password: &str, stored_hash: &str) -> Result<bool, HashError> {
    let parsed_hash = PasswordHash::new(stored_hash)?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
