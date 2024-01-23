//! Implements account component

use axum::async_trait;
use sqlx::{prelude::FromRow, PgPool};

/// Represents an account
#[derive(Debug, Clone, FromRow)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub default_asset: Option<String>,
}

/// Represents an account repository
#[async_trait]
pub trait AccountRepository: Sync + Send {
    /// Creates an account
    async fn create_account(&self, name: String, default_asset: Option<String>) -> Option<Account>;

    /// Find all accounts by IDs
    async fn find_all_accounts_by_ids(&self, ids: Vec<String>) -> Option<Vec<Account>>;
}

#[async_trait]
impl AccountRepository for PgPool {
    async fn create_account(&self, name: String, default_asset: Option<String>) -> Option<Account> {
        sqlx::query_as::<_, Account>("insert into accounts (id, name, default_asset) values ($1, $2, $3) returning id, name, default_asset")
            .bind(nanoid::nanoid!())
            .bind(name)
            .bind(default_asset)
            .fetch_one(self)
            .await
            .ok()
    }

    async fn find_all_accounts_by_ids(&self, ids: Vec<String>) -> Option<Vec<Account>> {
        sqlx::query_as::<_, Account>(
            "select id, name, default_asset from accounts where id = ANY($1) order by name",
        )
        .bind(&ids[..])
        .fetch_all(self)
        .await
        .ok()
    }
}

