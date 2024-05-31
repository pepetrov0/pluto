//! This module provides the implementation for managing users within our
//! PostgreSQL data source.

use axum::async_trait;

use crate::domain::{
    database::{
        users::{User, Users},
        Error, Result,
    },
    identifier::Id,
};

use super::PgTransaction;

/// A trait describing a repository of users.
#[async_trait]
impl Users for PgTransaction {
    /// Finds a user by identifier.
    #[tracing::instrument(err, skip(self))]
    async fn find_user_by_id(&mut self, id: Id) -> Result<Option<User>> {
        sqlx::query_as(include_str!("find_user_by_id.sql"))
            .bind(id)
            .fetch_optional(&mut *self.0)
            .await
            .map_err(Error::from)
    }

    /// Finds a user by email.
    #[tracing::instrument(err, skip(self))]
    async fn find_user_by_email(&mut self, email: &str) -> Result<Option<User>> {
        sqlx::query_as(include_str!("find_user_by_email.sql"))
            .bind(email)
            .fetch_optional(&mut *self.0)
            .await
            .map_err(Error::from)
    }

    /// Create a user.
    #[tracing::instrument(err, skip(self, password))]
    async fn create_user(&mut self, email: &str, password: Option<&str>) -> Result<User> {
        sqlx::query_as(include_str!("create_user.sql"))
            .bind(email)
            .bind(password)
            .fetch_one(&mut *self.0)
            .await
            .map_err(Error::from)
    }

    /// Update a user's email by their identifier.
    #[tracing::instrument(skip(self))]
    async fn update_user_email_by_id(&mut self, id: Id, new_email: &str) -> Result<Option<User>> {
        sqlx::query_as(include_str!("update_user_email_by_id.sql"))
            .bind(id)
            .bind(new_email)
            .fetch_optional(&mut *self.0)
            .await
            .map_err(Error::from)
    }

    /// Update a user's password by their identifier.
    #[tracing::instrument(skip(self))]
    async fn update_user_password_by_id(
        &mut self,
        id: Id,
        new_password: Option<&str>,
    ) -> Result<Option<User>> {
        sqlx::query_as(include_str!("update_user_password_by_id.sql"))
            .bind(id)
            .bind(new_password)
            .fetch_optional(&mut *self.0)
            .await
            .map_err(Error::from)
    }
}
