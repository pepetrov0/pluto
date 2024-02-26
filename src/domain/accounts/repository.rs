use axum::async_trait;

use crate::core::database::{
    IntoRepositoryResult, ReadonlyDatabaseRepository, RepositoryResult, WriteDatabaseRepository,
};

use super::Account;

#[async_trait]
pub trait AccountReadonlyRepository {
    async fn list_accounts(&mut self) -> RepositoryResult<Vec<Account>>;

    async fn list_accounts_by_ids(&mut self, ids: &[String]) -> RepositoryResult<Vec<Account>>;
}

#[async_trait]
pub trait AccountWriteRepository {
    async fn create_account(&mut self, name: &str) -> RepositoryResult<Account>;
}

#[async_trait]
impl<T> AccountReadonlyRepository for T
where
    T: ReadonlyDatabaseRepository + Send + std::fmt::Debug,
{
    #[tracing::instrument(err)]
    async fn list_accounts(&mut self) -> RepositoryResult<Vec<Account>> {
        sqlx::query_as::<_, Account>("select id, name from accounts order by name")
            .fetch_all(self.acquire().await?)
            .await
            .into_repository_result()
    }

    #[tracing::instrument(err)]
    async fn list_accounts_by_ids(&mut self, ids: &[String]) -> RepositoryResult<Vec<Account>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        sqlx::query_as::<_, Account>(
            "select id, name from accounts where id = ANY($1) order by name",
        )
        .bind(ids)
        .fetch_all(self.acquire().await?)
        .await
        .into_repository_result()
    }
}

#[async_trait]
impl<T> AccountWriteRepository for T
where
    T: WriteDatabaseRepository + Send + std::fmt::Debug,
{
    #[tracing::instrument(err)]
    async fn create_account(&mut self, name: &str) -> RepositoryResult<Account> {
        sqlx::query_as::<_, Account>(
            "insert into accounts (id, name) values ($1, $2) returning id, name",
        )
        .bind(nanoid::nanoid!())
        .bind(name)
        .fetch_one(self.acquire().await?)
        .await
        .into_repository_result()
    }
}
