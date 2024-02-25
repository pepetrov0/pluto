use axum::{async_trait, extract::FromRequestParts, http::request::Parts, Extension};
use sqlx::prelude::FromRow;

use crate::{
    core::database::{ReadonlyDatabaseRepository, WriteDatabaseRepository},
    AppState,
};

#[derive(Debug, Clone, FromRow)]
pub struct Session {
    pub id: String,

    pub usr: String,
}

#[async_trait]
pub trait SessionReadonlyRepository {
    async fn find_session(&mut self, id: &str) -> Option<Session>;
}

#[async_trait]
pub trait SessionWriteRepository {
    async fn create_session(&mut self, user: String) -> Option<Session>;

    async fn delete_session(&mut self, id: &str) -> Option<()>;
}

#[async_trait]
impl<T> SessionReadonlyRepository for T
where
    T: ReadonlyDatabaseRepository + Send + std::fmt::Debug,
{
    #[tracing::instrument]
    async fn find_session(&mut self, id: &str) -> Option<Session> {
        sqlx::query_as::<_, Session>("select id, usr from sessions where id=$1")
            .bind(id)
            .fetch_one(self.acquire().await.ok()?)
            .await
            .map_err(|v| tracing::warn!("{:#?}", v))
            .ok()
    }
}

#[async_trait]
impl<T> SessionWriteRepository for T
where
    T: WriteDatabaseRepository + Send + std::fmt::Debug,
{
    #[tracing::instrument]
    async fn create_session(&mut self, user: String) -> Option<Session> {
        sqlx::query_as::<_, Session>(
            "insert into sessions (id, usr) values ($1, $2) returning id, usr",
        )
        .bind(nanoid::nanoid!())
        .bind(user)
        .fetch_one(self.acquire().await.ok()?)
        .await
        .map_err(|v| tracing::warn!("{:#?}", v))
        .ok()
    }

    #[tracing::instrument]
    async fn delete_session(&mut self, id: &str) -> Option<()> {
        sqlx::query("delete from sessions where id=$1")
            .bind(id)
            .execute(self.acquire().await.ok()?)
            .await
            .map_err(|v| tracing::warn!("{:#?}", v))
            .ok()?;
        Some(())
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
