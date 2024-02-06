//! Implements account component

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
pub trait AccountRepository: Sync + Send {
    /// Creates an account
    async fn create_account(&self, name: String) -> Option<Account>;

    /// List all accounts
    async fn list_accounts(&self) -> Option<Vec<Account>>;

    /// List all accounts by IDs
    async fn list_accounts_by_ids(&self, ids: Vec<String>) -> Option<Vec<Account>>;
}

#[async_trait]
impl AccountRepository for PgPool {
    #[tracing::instrument]
    async fn create_account(&self, name: String) -> Option<Account> {
        sqlx::query_as::<_, Account>("insert into accounts (id, name) values ($1, $2) returning id, name")
            .bind(nanoid::nanoid!())
            .bind(name)
            .fetch_one(self)
            .await
            .ok()
    }

    #[tracing::instrument]
    async fn list_accounts(&self) -> Option<Vec<Account>> {
        sqlx::query_as::<_, Account>("select id, name from accounts order by name")
            .fetch_all(self)
            .await
            .ok()
    }

    #[tracing::instrument]
    async fn list_accounts_by_ids(&self, ids: Vec<String>) -> Option<Vec<Account>> {
        if ids.is_empty() {
            return Some(vec![]);
        }

        sqlx::query_as::<_, Account>(
            "select id, name from accounts where id = ANY($1) order by name",
        )
        .bind(&ids[..])
        .fetch_all(self)
        .await
        .ok()
    }
}
