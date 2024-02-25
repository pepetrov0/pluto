use axum::async_trait;

use crate::core::database::WriteDatabaseRepository;

use super::CsrfToken;

#[async_trait]
pub trait CsrfTokenRepository {
    async fn consume_csrf_token(&mut self, id: &str) -> Option<CsrfToken>;

    async fn create_csrf_token(&mut self, user: &str, usage: &str) -> Option<CsrfToken>;
}

#[async_trait]
impl<T> CsrfTokenRepository for T
where
    T: WriteDatabaseRepository + Send + std::fmt::Debug,
{
    async fn consume_csrf_token(&mut self, id: &str) -> Option<CsrfToken> {
        sqlx::query_as::<_, CsrfToken>(
            "delete from valid_csrf_tokens where id=$1 returning id, usr, usage",
        )
        .bind(id)
        .fetch_one(self.acquire().await.ok()?)
        .await
        .map_err(|v| tracing::warn!("{:#?}", v))
        .ok()
    }

    async fn create_csrf_token(&mut self, user: &str, usage: &str) -> Option<CsrfToken> {
        sqlx::query_as::<_, CsrfToken>(
            "insert into csrf_tokens (id, usr, usage) values ($1, $2, $3) returning id, usr, usage",
        )
        .bind(nanoid::nanoid!())
        .bind(user)
        .bind(usage)
        .fetch_one(self.acquire().await.ok()?)
        .await
        .map_err(|v| tracing::warn!("{:#?}", v))
        .ok()
    }
}
