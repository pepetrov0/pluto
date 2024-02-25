use axum::async_trait;
use sqlx::FromRow;

use crate::core::database::{ReadonlyDatabaseRepository, WriteDatabaseRepository};

#[derive(Debug, Clone, FromRow)]
pub struct AccountOwnership {
    pub id: i32,
    pub usr: String,
    pub account: String,
}

#[async_trait]
pub trait AccountOwnershipReadonlyRepository {
    async fn list_account_ownerships(&mut self) -> Option<Vec<AccountOwnership>>;

    async fn list_account_ownerships_by_user_or_account(
        &mut self,
        user_or_account: &str,
    ) -> Option<Vec<AccountOwnership>>;

    async fn list_account_ownerships_by_users_or_accounts(
        &mut self,
        ids: Vec<String>,
    ) -> Option<Vec<AccountOwnership>>;
}

#[async_trait]
pub trait AccountOwnershipWriteRepository {
    async fn create_account_ownership(
        &mut self,
        user: &str,
        account: &str,
    ) -> Option<AccountOwnership>;
}

#[async_trait]
impl<T> AccountOwnershipReadonlyRepository for T
where
    T: ReadonlyDatabaseRepository + Send + std::fmt::Debug,
{
    #[tracing::instrument]
    async fn list_account_ownerships(&mut self) -> Option<Vec<AccountOwnership>> {
        sqlx::query_as::<_, AccountOwnership>("select id, usr, account from accounts_ownerships")
            .fetch_all(self.acquire().await.ok()?)
            .await
            .map_err(|v| tracing::warn!("{:#?}", v))
            .ok()
    }

    #[tracing::instrument]
    async fn list_account_ownerships_by_user_or_account(
        &mut self,
        user_or_account: &str,
    ) -> Option<Vec<AccountOwnership>> {
        sqlx::query_as::<_, AccountOwnership>(
            "select id, usr, account from accounts_ownerships where usr=$1 or account=$1",
        )
        .bind(user_or_account)
        .fetch_all(self.acquire().await.ok()?)
        .await
        .map_err(|v| tracing::warn!("{:#?}", v))
        .ok()
    }

    #[tracing::instrument]
    async fn list_account_ownerships_by_users_or_accounts(
        &mut self,
        ids: Vec<String>,
    ) -> Option<Vec<AccountOwnership>> {
        if ids.is_empty() {
            return Some(vec![]);
        }

        sqlx::query_as::<_, AccountOwnership>(
            "select id, usr, account from accounts_ownerships where usr=ANY($1) or account=ANY($1)",
        )
        .bind(&ids[..])
        .fetch_all(self.acquire().await.ok()?)
        .await
        .map_err(|v| tracing::warn!("{:#?}", v))
        .ok()
    }
}

#[async_trait]
impl<T> AccountOwnershipWriteRepository for T
where
    T: WriteDatabaseRepository + Send + std::fmt::Debug,
{
    #[tracing::instrument]
    async fn create_account_ownership(
        &mut self,
        user: &str,
        account: &str,
    ) -> Option<AccountOwnership> {
        sqlx::query_as::<_, AccountOwnership>("insert into accounts_ownerships (usr, account) values ($1, $2) returning id, usr, account")
            .bind(user)
            .bind(account)
            .fetch_one(self.acquire().await.ok()?)
            .await
            .map_err(|v| tracing::warn!("{:#?}", v))
            .ok()
    }
}
