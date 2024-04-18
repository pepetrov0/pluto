use axum::async_trait;

use crate::domain::database::users::{User, Users};

use super::PgTransaction;

#[cfg(test)]
mod tests;

/// A trait describing a repository of users.
#[async_trait]
impl Users for PgTransaction {
    /// Finds a user by identifier.
    #[tracing::instrument(skip(self))]
    async fn find_user_by_id(&mut self, id: i32) -> Option<Option<User>> {
        sqlx::query_as(include_str!("find_user_by_id.sql"))
            .bind(id)
            .fetch_optional(&mut *self.0)
            .await
            .map_err(|e| tracing::warn!("error while fetching a user by identifier: {e:?}"))
            .ok()
    }

    /// Finds a user by email.
    #[tracing::instrument(skip(self))]
    async fn find_user_by_email(&mut self, email: &str) -> Option<Option<User>> {
        sqlx::query_as(include_str!("find_user_by_email.sql"))
            .bind(email)
            .fetch_optional(&mut *self.0)
            .await
            .map_err(|e| tracing::warn!("error while fetching a user by email: {e:?}"))
            .ok()
    }

    /// Create a user.
    #[tracing::instrument(skip(self, password))]
    async fn create_user(&mut self, email: &str, password: Option<&str>) -> Option<User> {
        sqlx::query_as(include_str!("create_user.sql"))
            .bind(email)
            .bind(password)
            .fetch_one(&mut *self.0)
            .await
            .map_err(|e| tracing::warn!("error creating a user: {e:?}"))
            .ok()
    }
}
