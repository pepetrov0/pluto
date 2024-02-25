use axum::async_trait;
use tracing::instrument;

use crate::core::database::{IntoRepositoryResult, RepositoryResult, WriteDatabaseRepository};

use super::CsrfToken;

#[async_trait]
pub trait CsrfTokenRepository {
    async fn consume_csrf_token(&mut self, id: &str) -> RepositoryResult<Option<CsrfToken>>;

    async fn create_csrf_token(&mut self, user: &str, usage: &str) -> RepositoryResult<CsrfToken>;
}

#[async_trait]
impl<T> CsrfTokenRepository for T
where
    T: WriteDatabaseRepository + Send + std::fmt::Debug,
{
    #[instrument(err)]
    async fn consume_csrf_token(&mut self, id: &str) -> RepositoryResult<Option<CsrfToken>> {
        sqlx::query_as::<_, CsrfToken>(
            "delete from valid_csrf_tokens where id=$1 returning id, usr, usage",
        )
        .bind(id)
        .fetch_optional(self.acquire().await?)
        .await
        .into_repository_result()
    }

    #[instrument(err)]
    async fn create_csrf_token(&mut self, user: &str, usage: &str) -> RepositoryResult<CsrfToken> {
        sqlx::query_as::<_, CsrfToken>(
            "insert into csrf_tokens (id, usr, usage) values ($1, $2, $3) returning id, usr, usage",
        )
        .bind(nanoid::nanoid!())
        .bind(user)
        .bind(usage)
        .fetch_one(self.acquire().await?)
        .await
        .into_repository_result()
    }
}
