//! A module providing a concept of a user.

use axum::async_trait;

use crate::domain::identifier::Id;

/// An entity representing a user.
#[derive(Debug, Clone, sqlx::FromRow, Eq, PartialEq)]
pub struct User {
    /// Identifier of the user.
    pub id: Id,
    /// Email.
    pub email: String,
    /// Argon hash of the password.
    pub password: Option<String>,
}

/// A trait describing a repository of users.
#[async_trait]
pub trait Users {
    /// Finds a user by identifier.
    ///
    /// _**NOTE:** The result of this function is `Option<Option<User>>`.
    /// The first `Option` would be `None` in case of an error, the second `Option`
    /// would be `None` if no user was found._
    async fn find_user_by_id(&mut self, id: Id) -> super::Result<Option<User>>;

    /// Finds a user by email.
    ///
    /// _**NOTE:** The result of this function is `Option<Option<User>>`.
    /// The first `Option` would be `None` in case of an error, the second `Option`
    /// would be `None` if no user was found._
    async fn find_user_by_email(&mut self, email: &str) -> super::Result<Option<User>>;

    /// Create a user.
    async fn create_user(&mut self, email: &str, password: Option<&str>) -> super::Result<User>;

    /// Update a user's email by their identifier.
    async fn update_user_email_by_id(
        &mut self,
        id: Id,
        new_email: &str,
    ) -> super::Result<Option<User>>;

    /// Update a user's password by their identifier.
    async fn update_user_password_by_id(&mut self, id: Id, new_password: Option<&str>) -> super::Result<Option<User>>;
}
