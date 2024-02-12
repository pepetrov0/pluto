//! Implements the entries component

use axum::async_trait;
use chrono::NaiveDateTime;
use sqlx::{prelude::FromRow, PgPool};

/// Represents an entry
#[derive(Debug, Clone, FromRow)]
pub struct Entry {
    pub id: String,
    pub note: String,
    pub account: String,
    pub asset: String,
    pub stamp: NaiveDateTime,
    pub amount: i64,
    pub settled: bool,
}
