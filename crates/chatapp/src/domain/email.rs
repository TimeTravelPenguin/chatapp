use std::{
    fmt::{self, Display},
    str::FromStr,
};

use email_address::EmailAddress;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserEmail(EmailAddress);

impl UserEmail {
    pub fn parse(input: &str) -> Result<Self, email_address::Error> {
        let original = EmailAddress::from_str(input.to_ascii_lowercase().trim())?;

        Ok(Self(original))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl Display for UserEmail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl AsRef<str> for UserEmail {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for UserEmail {
    type Err = email_address::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}
