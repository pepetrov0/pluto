//! Implements a session component

use axum::{async_trait, extract::FromRequestParts, http::request::Parts, Extension};
use sqlx::{prelude::FromRow, Postgres};

use crate::{
    database::{AsReadonlyExecutor, AsWriteExecutor},
    AppState,
};

/// Represents a session
#[derive(Debug, Clone, FromRow)]
pub struct Session {
    /// The session id
    pub id: String,
    /// A user id or email
    pub usr: String,
}

/// A session readonly repository
#[async_trait]
pub trait SessionReadonlyRepository {
    /// Finds a session
    async fn find_session(&mut self, id: &str) -> Option<Session>;
}

/// A session write repository
#[async_trait]
pub trait SessionWriteRepository {
    /// Creates a session
    async fn create_session(&mut self, user: String) -> Option<Session>;

    /// Deletes a session
    async fn delete_session(&mut self, id: &str);
}

#[async_trait]
impl<T> SessionReadonlyRepository for T
where
    T: AsReadonlyExecutor<Postgres> + Send + std::fmt::Debug,
{
    #[tracing::instrument]
    async fn find_session(&mut self, id: &str) -> Option<Session> {
        sqlx::query_as::<_, Session>("select id, usr from sessions where id=$1")
            .bind(id)
            .fetch_one(self.as_executor())
            .await
            .map_err(|v| tracing::warn!("{:#?}", v))
            .ok()
    }
}

#[async_trait]
impl<T> SessionWriteRepository for T
where
    T: AsWriteExecutor<Postgres> + Send + std::fmt::Debug,
{
    #[tracing::instrument]
    async fn create_session(&mut self, user: String) -> Option<Session> {
        sqlx::query_as::<_, Session>(
            "insert into sessions (id, usr) values ($1, $2) returning id, usr",
        )
        .bind(nanoid::nanoid!())
        .bind(user)
        .fetch_one(self.as_executor())
        .await
        .map_err(|v| tracing::warn!("{:#?}", v))
        .ok()
    }

    #[tracing::instrument]
    async fn delete_session(&mut self, id: &str) {
        let _ = sqlx::query("delete from sessions where id=$1")
            .bind(id)
            .execute(self.as_executor())
            .await
            .map_err(|v| tracing::warn!("{:#?}", v));
    }
}

#[async_trait]
impl FromRequestParts<AppState> for Session {
    type Rejection = <Extension<Session> as FromRequestParts<AppState>>::Rejection;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Extension::from_request_parts(parts, state)
            .await
            .map(|v| v.0)
    }
}
