use std::{
    fmt::{self, Display},
    str::FromStr,
};

use thiserror::Error;
use time::OffsetDateTime;

use crate::domain::email::UserEmail;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Username cannot be empty")]
    EmptyUsername,
    #[error("Username must only contain alphanumeric characters, underscores, or hyphens: {0}")]
    InvalidUsername(String),
    #[error("Invalid email: {0}")]
    InvalidEmail(email_address::Error),
}

#[derive(Debug, Clone)]
pub struct User {
    pub user_name: String,
    pub display_name: String,
    pub email: UserEmail,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Username(String);

impl Username {
    pub fn parse(input: &str) -> Result<Self, UserError> {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return Err(UserError::EmptyUsername);
        }

        if !trimmed
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(UserError::InvalidUsername(trimmed.to_string()));
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for Username {
    type Err = UserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn valid_usernames() {
        let valid_usernames = ["alice", "bob_123", "charlie-xyz"];

        for &username in &valid_usernames {
            let parsed = Username::parse(username);
            assert!(parsed.is_ok(), "Expected '{}' to be valid", username);
            assert_eq!(parsed.unwrap().as_str(), username);
        }
    }

    #[test]
    fn invalid_usernames() {
        let invalid_usernames = ["", "   ", "invalid username", "user!name", "user@name"];

        for &username in &invalid_usernames {
            let parsed = Username::parse(username);
            assert!(parsed.is_err(), "Expected '{}' to be invalid", username);
        }
    }
}
