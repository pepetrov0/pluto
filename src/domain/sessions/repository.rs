use axum::async_trait;

use crate::core::database::{
    IntoRepositoryResult, ReadonlyDatabaseRepository, RepositoryResult, WriteDatabaseRepository,
};

use super::Session;

#[async_trait]
pub trait SessionReadonlyRepository {
    async fn find_session(&mut self, id: &str) -> RepositoryResult<Option<Session>>;
}

#[async_trait]
pub trait SessionWriteRepository {
    async fn create_session(&mut self, user: &str) -> RepositoryResult<Session>;
    async fn delete_session(&mut self, id: &str) -> RepositoryResult<()>;
}

#[async_trait]
impl<T> SessionReadonlyRepository for T
where
    T: ReadonlyDatabaseRepository + Send + std::fmt::Debug,
{
    #[tracing::instrument(err)]
    async fn find_session(&mut self, id: &str) -> RepositoryResult<Option<Session>> {
        sqlx::query_as::<_, Session>("select id, usr from sessions where id=$1")
            .bind(id)
            .fetch_optional(self.acquire().await?)
            .await
            .into_repository_result()
    }
}

#[async_trait]
impl<T> SessionWriteRepository for T
where
    T: WriteDatabaseRepository + Send + std::fmt::Debug,
{
    #[tracing::instrument(err)]
    async fn create_session(&mut self, user: &str) -> RepositoryResult<Session> {
        sqlx::query_as::<_, Session>(
            "insert into sessions (id, usr) values ($1, $2) returning id, usr",
        )
        .bind(nanoid::nanoid!())
        .bind(user)
        .fetch_one(self.acquire().await?)
        .await
        .into_repository_result()
    }

    #[tracing::instrument(err)]
    async fn delete_session(&mut self, id: &str) -> RepositoryResult<()> {
        sqlx::query("delete from sessions where id=$1")
            .bind(id)
            .execute(self.acquire().await?)
            .await
            .map(|_| ())
            .into_repository_result()
    }
}
