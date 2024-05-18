//! This module provides the implementation for managing sessions within our
//! PostgreSQL data source.

use axum::async_trait;

use crate::domain::{
    database::{
        sessions::{Session, Sessions},
        Error, Result,
    },
    identifier::Id,
};

use super::PgTransaction;

#[async_trait]
impl Sessions for PgTransaction {
    #[tracing::instrument(err, skip(self))]
    async fn find_session_by_id(&mut self, id: Id) -> Result<Option<Session>> {
        sqlx::query_as(include_str!("find_session_by_id.sql"))
            .bind(id)
            .fetch_optional(&mut *self.0)
            .await
            .map_err(Error::from)
    }

    #[tracing::instrument(err, skip(self))]
    async fn find_all_sessions_by_user_id(&mut self, user_id: Id) -> Result<Vec<Session>> {
        sqlx::query_as(include_str!("find_all_sessions_by_user_id.sql"))
            .bind(user_id)
            .fetch_all(&mut *self.0)
            .await
            .map_err(Error::from)
    }

    #[tracing::instrument(err, skip(self))]
    async fn create_session(&mut self, user_id: Id, agent: &str) -> Result<Session> {
        sqlx::query_as(include_str!("create_session.sql"))
            .bind(user_id)
            .bind(agent)
            .fetch_one(&mut *self.0)
            .await
            .map_err(Error::from)
    }

    #[tracing::instrument(err, skip(self))]
    async fn delete_session_by_id(&mut self, id: Id) -> Result<()> {
        sqlx::query(include_str!("delete_session_by_id.sql"))
            .bind(id)
            .execute(&mut *self.0)
            .await
            .map(|_| ())
            .map_err(Error::from)
    }

    #[tracing::instrument(err, skip(self))]
    async fn delete_all_sessions_by_user_id(&mut self, user_id: Id) -> Result<()> {
        sqlx::query(include_str!("delete_all_sessions_by_user_id.sql"))
            .bind(user_id)
            .execute(&mut *self.0)
            .await
            .map(|_| ())
            .map_err(Error::from)
    }
}
