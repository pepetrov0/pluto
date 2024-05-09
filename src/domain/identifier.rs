//! This module includes an implementation of a common identifier.

use std::fmt::Display;

use sqlx::prelude::Type;

/// An identifier
#[derive(Debug, Type, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[sqlx(transparent)]
pub struct Id(i64);

impl From<i64> for Id {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl TryFrom<&str> for Id {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        crockford::decode(value)
            .map(|v| (v as i64).into())
            .map_err(|_| ())
    }
}

impl TryFrom<String> for Id {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl TryFrom<&String> for Id {
    type Error = ();

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let encoded = crockford::encode(self.0 as u64);
        write!(f, "{encoded}")
    }
}

#[cfg(test)]
mod tests {
    use super::Id;

    #[test]
    fn encoding() {
        let id: Id = 5111.into();
        assert_eq!(id.to_string(), "4ZQ");
    }

    #[test]
    fn decoding() {
        let id: Id = "4zq".try_into().unwrap();
        assert_eq!(id, 5111.into());
    }

    #[test]
    fn decoding_gibberish() {
        Id::try_from("1(#%)]").unwrap_err();
    }
}
