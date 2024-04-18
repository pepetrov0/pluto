use axum::async_trait;

use crate::domain::database::users::{User, Users};

use super::AnyTransaction;

/// A trait describing a repository of users.
#[async_trait]
impl Users for AnyTransaction {
    /// Finds a user by identifier.
    #[tracing::instrument(skip(self))]
    async fn find_user_by_id(&mut self, id: i32) -> Option<Option<User>> {
        match self {
            AnyTransaction::Sqlite(v) => v.find_user_by_id(id).await,
            AnyTransaction::Pg(v) => v.find_user_by_id(id).await,
        }
    }

    /// Finds a user by email.
    #[tracing::instrument(skip(self))]
    async fn find_user_by_email(&mut self, email: &str) -> Option<Option<User>> {
        match self {
            AnyTransaction::Sqlite(v) => v.find_user_by_email(email).await,
            AnyTransaction::Pg(v) => v.find_user_by_email(email).await,
        }
    }

    /// Create a user.
    #[tracing::instrument(skip(self))]
    async fn create_user(&mut self, email: &str, password: Option<&str>) -> Option<User> {
        match self {
            AnyTransaction::Sqlite(v) => v.create_user(email, password).await,
            AnyTransaction::Pg(v) => v.create_user(email, password).await,
        }
    }
}