//! This module provides the implementation for managing users within our
//! SQLite data source.

use axum::async_trait;
use sqlx::prelude::FromRow;

use crate::domain::{
    identifier::Id,
    users::{User, UserError, UsersRepository},
};

use super::SqliteTransaction;

#[derive(Debug, FromRow)]
struct UserE {
    pub id: Id,
    pub email: String,
    pub password: Option<String>,
}

/// A trait describing a repository of users.
#[async_trait]
impl UsersRepository for SqliteTransaction {
    /// Finds a user by identifier.
    #[tracing::instrument(err, skip(self))]
    async fn find_user_by_id(&mut self, id: Id) -> Result<User, UserError> {
        sqlx::query_as(include_str!("find_user_by_id.sql"))
            .bind(id)
            .fetch_one(&mut *self.0)
            .await
            .map(UserE::into)
            .map_err(UserError::from)
    }

    /// Finds a user by email.
    #[tracing::instrument(err, skip(self))]
    async fn find_user_by_email(&mut self, email: &str) -> Result<User, UserError> {
        sqlx::query_as(include_str!("find_user_by_email.sql"))
            .bind(email)
            .fetch_one(&mut *self.0)
            .await
            .map(UserE::into)
            .map_err(UserError::from)
    }

    /// Create a user.
    #[tracing::instrument(err, skip(self, password))]
    async fn create_user(
        &mut self,
        email: &str,
        password: Option<&str>,
    ) -> Result<User, UserError> {
        sqlx::query_as(include_str!("create_user.sql"))
            .bind(email)
            .bind(password)
            .fetch_one(&mut *self.0)
            .await
            .map(UserE::into)
            .map_err(UserError::from)
    }

    /// Update a user's email by their identifier.
    #[tracing::instrument(skip(self))]
    async fn update_user_email_by_id(
        &mut self,
        id: Id,
        new_email: &str,
    ) -> Result<User, UserError> {
        sqlx::query_as(include_str!("update_user_email_by_id.sql"))
            .bind(id)
            .bind(new_email)
            .fetch_one(&mut *self.0)
            .await
            .map(UserE::into)
            .map_err(UserError::from)
    }

    /// Update a user's password by their identifier.
    #[tracing::instrument(skip(self))]
    async fn update_user_password_by_id(
        &mut self,
        id: Id,
        new_password: Option<&str>,
    ) -> Result<User, UserError> {
        sqlx::query_as(include_str!("update_user_password_by_id.sql"))
            .bind(id)
            .bind(new_password)
            .fetch_one(&mut *self.0)
            .await
            .map(UserE::into)
            .map_err(UserError::from)
    }
}

impl Into<User> for UserE {
    fn into(self) -> User {
        User {
            id: self.id,
            email: self.email,
            password: self.password,
        }
    }
}
