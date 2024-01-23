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

    async fn find_account_ownerships_by_user(&self, user: &str) -> Option<Vec<AccountOwnership>>;
}

#[async_trait]
impl AccountOwnershipRepository for PgPool {
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

    async fn find_account_ownerships_by_user(&self, user: &str) -> Option<Vec<AccountOwnership>> {
        sqlx::query_as::<_, AccountOwnership>(
            "select id, usr, account from accounts_ownerships where usr=$1",
        )
        .bind(user)
        .fetch_all(self)
        .await
        .ok()
    }
}
