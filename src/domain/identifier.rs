//! This module includes an implementation of a common identifier.

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
