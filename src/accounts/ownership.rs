//! Implements account ownership

use axum::async_trait;
use sqlx::{FromRow, PgPool};

/// Represents an account ownership
#[derive(Debug, Clone, FromRow)]
pub struct AccountOwnership {
    pub id: i32,
    pub usr: String,
    pub account: String,
}

/// Represents an account ownership repository
#[async_trait]
pub trait AccountOwnershipRepository: Sync + Send {
    /// Creates an account ownership
    async fn create_account_ownership(
        &self,
        user: String,
        account: String,
    ) -> Option<AccountOwnership>;

    /// List all account ownerships
    async fn list_account_ownerships(&self) -> Option<Vec<AccountOwnership>>;

    /// List all account ownerships for a user
    async fn list_account_ownerships_by_user(&self, user: &str) -> Option<Vec<AccountOwnership>>;

    /// List all account ownerships for accounts
    async fn list_account_ownerships_by_accounts(
        &self,
        ids: Vec<String>,
    ) -> Option<Vec<AccountOwnership>>;
}

#[async_trait]
impl AccountOwnershipRepository for PgPool {
    #[tracing::instrument]
    async fn create_account_ownership(
        &self,
        user: String,
        account: String,
    ) -> Option<AccountOwnership> {
        sqlx::query_as::<_, AccountOwnership>("insert into accounts_ownerships (usr, account) values ($1, $2) returning id, usr, account")
            .bind(user)
            .bind(account)
            .fetch_one(self)
            .await
            .ok()
    }

    #[tracing::instrument]
    async fn list_account_ownerships(&self) -> Option<Vec<AccountOwnership>> {
        sqlx::query_as::<_, AccountOwnership>(
            "select id, usr, account from accounts_ownerships",
        )
        .fetch_all(self)
        .await
        .ok()
    }

    #[tracing::instrument]
    async fn list_account_ownerships_by_user(&self, user: &str) -> Option<Vec<AccountOwnership>> {
        sqlx::query_as::<_, AccountOwnership>(
            "select id, usr, account from accounts_ownerships where usr=$1",
        )
        .bind(user)
        .fetch_all(self)
        .await
        .ok()
    }

    #[tracing::instrument]
    async fn list_account_ownerships_by_accounts(
        &self,
        ids: Vec<String>,
    ) -> Option<Vec<AccountOwnership>> {
        if ids.is_empty() {
            return Some(vec![]);
        }

        sqlx::query_as::<_, AccountOwnership>(
            "select id, usr, account from accounts_ownerships where account=ANY($1)",
        )
        .bind(&ids[..])
        .fetch_all(self)
        .await
        .ok()
    }
}
