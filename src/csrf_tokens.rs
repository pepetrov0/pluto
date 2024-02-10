//! Implements CSRF tokens

use axum::async_trait;
use sqlx::{prelude::FromRow, Postgres};

use crate::database::AsWriteExecutor;

/// Represents a CSRF token
#[derive(Debug, Clone, FromRow)]
pub struct CsrfToken {
    pub id: String,
    pub usr: String,
    pub usage: String,
}

/// Represents a CSRF token repository
#[async_trait]
pub trait CsrfTokenRepository {
    /// Find and remove a CSRF token
    async fn consume_csrf_token(&mut self, id: &str) -> Option<CsrfToken>;

    /// Create a CSRF token
    async fn create_csrf_token(&mut self, user: &str, usage: &str) -> Option<CsrfToken>;
}

#[async_trait]
impl<T> CsrfTokenRepository for T
where
    T: AsWriteExecutor<Postgres> + Send + std::fmt::Debug,
{
    /// Find and remove a CSRF token
    async fn consume_csrf_token(&mut self, id: &str) -> Option<CsrfToken> {
        sqlx::query_as::<_, CsrfToken>(
            "delete from valid_csrf_tokens where id=$1 returning id, usr, usage",
        )
        .bind(id)
        .fetch_one(self.as_executor())
        .await
        .map_err(|v| tracing::error!("{:#?}", v))
        .ok()
    }

    /// Create a CSRF token
    async fn create_csrf_token(&mut self, user: &str, usage: &str) -> Option<CsrfToken> {
        sqlx::query_as::<_, CsrfToken>(
            "insert into csrf_tokens (id, usr, usage) values ($1, $2, $3) returning id, usr, usage",
        )
        .bind(nanoid::nanoid!())
        .bind(user)
        .bind(usage)
        .fetch_one(self.as_executor())
        .await
        .map_err(|v| tracing::error!("{:#?}", v))
        .ok()
    }
}
