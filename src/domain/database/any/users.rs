use axum::async_trait;

use crate::domain::{
    identifier::Id,
    users::{User, UserError, UsersRepository},
};

use super::AnyTransaction;

/// A trait describing a repository of users.
#[async_trait]
impl UsersRepository for AnyTransaction {
    /// Finds a user by identifier.
    #[tracing::instrument(skip(self))]
    async fn find_user_by_id(&mut self, id: Id) -> Result<User, UserError> {
        match self {
            AnyTransaction::Sqlite(v) => v.find_user_by_id(id).await,
            AnyTransaction::Pg(v) => v.find_user_by_id(id).await,
        }
    }

    /// Finds a user by email.
    #[tracing::instrument(skip(self))]
    async fn find_user_by_email(&mut self, email: &str) -> Result<User, UserError> {
        match self {
            AnyTransaction::Sqlite(v) => v.find_user_by_email(email).await,
            AnyTransaction::Pg(v) => v.find_user_by_email(email).await,
        }
    }

    /// Create a user.
    #[tracing::instrument(skip(self, password))]
    async fn create_user(
        &mut self,
        email: &str,
        password: Option<&str>,
    ) -> Result<User, UserError> {
        match self {
            AnyTransaction::Sqlite(v) => v.create_user(email, password).await,
            AnyTransaction::Pg(v) => v.create_user(email, password).await,
        }
    }

    /// Update a user's email by their identifier.
    #[tracing::instrument(skip(self))]
    async fn update_user_email_by_id(
        &mut self,
        id: Id,
        new_email: &str,
    ) -> Result<User, UserError> {
        match self {
            AnyTransaction::Sqlite(v) => v.update_user_email_by_id(id, new_email).await,
            AnyTransaction::Pg(v) => v.update_user_email_by_id(id, new_email).await,
        }
    }

    /// Update a user's password by their identifier.
    #[tracing::instrument(skip(self))]
    async fn update_user_password_by_id(
        &mut self,
        id: Id,
        new_password: Option<&str>,
    ) -> Result<User, UserError> {
        match self {
            AnyTransaction::Sqlite(v) => v.update_user_password_by_id(id, new_password).await,
            AnyTransaction::Pg(v) => v.update_user_password_by_id(id, new_password).await,
        }
    }
}
