//! Implements account component

use axum::async_trait;
use sqlx::{prelude::FromRow};

use crate::database::{ReadonlyDatabaseRepository, WriteDatabaseRepository};

/// Represents an account
#[derive(Debug, Clone, FromRow)]
pub struct Account {
    pub id: String,
    pub name: String,
}

/// Represents an account readonly repository
#[async_trait]
pub trait AccountReadonlyRepository {
    /// List all accounts
    async fn list_accounts(&mut self) -> Option<Vec<Account>>;

    /// List all accounts by IDs
    async fn list_accounts_by_ids(&mut self, ids: Vec<String>) -> Option<Vec<Account>>;
}

/// Represents an account write repository
#[async_trait]
pub trait AccountWriteRepository {
    /// Creates an account
    async fn create_account(&mut self, name: &str) -> Option<Account>;
}

#[async_trait]
impl<T> AccountReadonlyRepository for T
where
    T: ReadonlyDatabaseRepository + Send + std::fmt::Debug,
{
    #[tracing::instrument]
    async fn list_accounts(&mut self) -> Option<Vec<Account>> {
        sqlx::query_as::<_, Account>("select id, name from accounts order by name")
            .fetch_all(self.acquire().await?)
            .await
            .map_err(|v| tracing::warn!("{:#?}", v))
            .ok()
    }

    #[tracing::instrument]
    async fn list_accounts_by_ids(&mut self, ids: Vec<String>) -> Option<Vec<Account>> {
        if ids.is_empty() {
            return Some(vec![]);
        }

        sqlx::query_as::<_, Account>(
            "select id, name from accounts where id = ANY($1) order by name",
        )
        .bind(&ids[..])
        .fetch_all(self.acquire().await?)
        .await
        .map_err(|v| tracing::warn!("{:#?}", v))
        .ok()
    }
}

#[async_trait]
impl<T> AccountWriteRepository for T
where
    T: WriteDatabaseRepository + Send + std::fmt::Debug,
{
    #[tracing::instrument]
    async fn create_account(&mut self, name: &str) -> Option<Account> {
        sqlx::query_as::<_, Account>(
            "insert into accounts (id, name) values ($1, $2) returning id, name",
        )
        .bind(nanoid::nanoid!())
        .bind(name)
        .fetch_one(self.acquire().await?)
        .await
        .map_err(|v| tracing::warn!("{:#?}", v))
        .ok()
    }
}
