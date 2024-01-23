//! Implements account component

use axum::async_trait;
use sqlx::{prelude::FromRow, PgPool};

/// Represents an account
#[derive(Debug, Clone, FromRow)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub default_asset: Option<String>
}

/// Represents an account ownership
#[derive(Debug, Clone, FromRow)]
pub struct AccountOwnership {
    pub id: String,
    pub usr: String,
    pub account: String,
}

/// Represents an account repository
#[async_trait]
pub trait AccountRepository: Sync + Send {}

/// Represents an account ownership repository
#[async_trait]
pub trait AccountOwnershipRepository: Sync + Send {}

#[async_trait]
impl AccountRepository for PgPool {}

#[async_trait]
impl AccountOwnershipRepository for PgPool {}
