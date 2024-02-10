//! Implements account ownership

use axum::async_trait;
use sqlx::{FromRow, Postgres};

use crate::database::AsExecutor;

/// Represents an account ownership
#[derive(Debug, Clone, FromRow)]
pub struct AccountOwnership {
    pub id: i32,
    pub usr: String,
    pub account: String,
}

/// Represents an account ownership repository
#[async_trait]
pub trait AccountOwnershipRepository {
    /// Creates an account ownership
    async fn create_account_ownership(
        &mut self,
        user: String,
        account: String,
    ) -> Option<AccountOwnership>;

    /// List all account ownerships
    async fn list_account_ownerships(&mut self) -> Option<Vec<AccountOwnership>>;

    /// List all account ownerships for a user
    async fn list_account_ownerships_by_user(
        &mut self,
        user: &str,
    ) -> Option<Vec<AccountOwnership>>;

    /// List all account ownerships for accounts
    async fn list_account_ownerships_by_accounts(
        &mut self,
        ids: Vec<String>,
    ) -> Option<Vec<AccountOwnership>>;
}

#[async_trait]
impl<T> AccountOwnershipRepository for T
where
    T: AsExecutor<Postgres> + Send + std::fmt::Debug,
{
    #[tracing::instrument]
    async fn create_account_ownership(
        &mut self,
        user: String,
        account: String,
    ) -> Option<AccountOwnership> {
        sqlx::query_as::<_, AccountOwnership>("insert into accounts_ownerships (usr, account) values ($1, $2) returning id, usr, account")
            .bind(user)
            .bind(account)
            .fetch_one(self.as_executor())
            .await
            .map_err(|v| tracing::error!("{:#?}", v))
            .ok()
    }

    #[tracing::instrument]
    async fn list_account_ownerships(&mut self) -> Option<Vec<AccountOwnership>> {
        sqlx::query_as::<_, AccountOwnership>("select id, usr, account from accounts_ownerships")
            .fetch_all(self.as_executor())
            .await
            .map_err(|v| tracing::error!("{:#?}", v))
            .ok()
    }

    #[tracing::instrument]
    async fn list_account_ownerships_by_user(
        &mut self,
        user: &str,
    ) -> Option<Vec<AccountOwnership>> {
        sqlx::query_as::<_, AccountOwnership>(
            "select id, usr, account from accounts_ownerships where usr=$1",
        )
        .bind(user)
        .fetch_all(self.as_executor())
        .await
        .map_err(|v| tracing::error!("{:#?}", v))
        .ok()
    }

    #[tracing::instrument]
    async fn list_account_ownerships_by_accounts(
        &mut self,
        ids: Vec<String>,
    ) -> Option<Vec<AccountOwnership>> {
        if ids.is_empty() {
            return Some(vec![]);
        }

        sqlx::query_as::<_, AccountOwnership>(
            "select id, usr, account from accounts_ownerships where account=ANY($1)",
        )
        .bind(&ids[..])
        .fetch_all(self.as_executor())
        .await
        .map_err(|v| tracing::error!("{:#?}", v))
        .ok()
    }
}
