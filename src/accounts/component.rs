use axum::async_trait;
use sqlx::{prelude::FromRow, PgPool};

/// Represents an account
#[derive(Debug, Clone, FromRow)]
pub struct Account {
    pub id: String,
    pub name: String,
}

/// Represents an account repository
#[async_trait]
pub trait AccountRepository: Sync + Send {}

/// Represents an account repository
#[async_trait]
impl AccountRepository for PgPool {}
