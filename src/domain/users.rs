//! This module implements the business logic for working with users.

use axum::async_trait;
use secrecy::Secret;

use crate::domain::identifier::Id;

/// An entity representing a user.
#[derive(Debug, Clone)]
pub struct User {
    /// Identifier of the user.
    pub id: Id,
    /// Email.
    pub email: String,
    /// Argon hash of the password.
    pub password: Option<Secret<String>>,
}

/// An error that might occur while working with users.
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum UserError {
    /// A general error described by a message.
    Message(String),
    /// An error indicating that no row was found by the query.
    NotFound,
}

/// A trait describing a repository of users.
#[async_trait]
pub trait UsersRepository {
    /// Finds a user by identifier.
    ///
    /// _**NOTE:** The result of this function is `Option<Option<User>>`.
    /// The first `Option` would be `None` in case of an error, the second `Option`
    /// would be `None` if no user was found._
    async fn find_user_by_id(&mut self, id: Id) -> Result<User, UserError>;

    /// Finds a user by email.
    ///
    /// _**NOTE:** The result of this function is `Option<Option<User>>`.
    /// The first `Option` would be `None` in case of an error, the second `Option`
    /// would be `None` if no user was found._
    async fn find_user_by_email(&mut self, email: &str) -> Result<User, UserError>;

    /// Create a user.
    async fn create_user(&mut self, email: &str, password: Option<&str>)
        -> Result<User, UserError>;

    /// Update a user's email by their identifier.
    async fn update_user_email_by_id(&mut self, id: Id, new_email: &str)
        -> Result<User, UserError>;

    /// Update a user's password by their identifier.
    async fn update_user_password_by_id(
        &mut self,
        id: Id,
        new_password: Option<&str>,
    ) -> Result<User, UserError>;
}

impl std::error::Error for UserError {}
impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<sqlx::Error> for UserError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Self::NotFound,
            e => Self::Message(format!("{e:?}")),
        }
    }
}

impl Eq for User {}
impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.email == other.email
    }
}
