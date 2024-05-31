//! This module implements the business logic for working with users.

use tracing::instrument;

use super::{
    database::{self, users::Users, AnyTransaction},
    identifier::Id,
};

/// A user.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct User {
    pub id: Id,
    pub email: String,
    pub has_password: bool,
}

/// A user with password.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserWithPassword {
    pub id: Id,
    pub email: String,
    pub password: Option<String>,
}

/// Finds a user by their identifier.
#[instrument(err, skip(tx))]
pub async fn find_user_by_id(tx: &mut AnyTransaction, id: Id) -> Result<User, UserError> {
    tx.find_user_by_id(id)
        .await
        .map_err(UserError::from)?
        .ok_or(UserError::UserNotFound)
        .map(User::from)
}

/// Finds a user by their email.
#[instrument(err, skip(tx))]
pub async fn find_user_by_email(tx: &mut AnyTransaction, email: &str) -> Result<User, UserError> {
    tx.find_user_by_email(email)
        .await
        .map_err(UserError::from)?
        .ok_or(UserError::UserNotFound)
        .map(User::from)
}

/// Finds a user by their identifier including password.
#[instrument(err, skip(tx))]
pub async fn find_user_with_password_by_id(
    tx: &mut AnyTransaction,
    id: Id,
) -> Result<UserWithPassword, UserError> {
    tx.find_user_by_id(id)
        .await
        .map_err(UserError::from)?
        .ok_or(UserError::UserNotFound)
        .map(UserWithPassword::from)
}

/// Finds a user by their email including password.
#[instrument(err, skip(tx))]
pub async fn find_user_with_password_by_email(
    tx: &mut AnyTransaction,
    email: &str,
) -> Result<UserWithPassword, UserError> {
    tx.find_user_by_email(email)
        .await
        .map_err(UserError::from)?
        .ok_or(UserError::UserNotFound)
        .map(UserWithPassword::from)
}

/// Creates a new user.
#[instrument(err, skip(tx, password))]
pub async fn create_user(
    tx: &mut AnyTransaction,
    email: &str,
    password: Option<&str>,
) -> Result<User, UserError> {
    tx.create_user(email, password)
        .await
        .map(User::from)
        .map_err(UserError::from)
}

/// Updates a user's email by their identifier.
#[instrument(err, skip(tx))]
pub async fn update_user_email_by_id(
    tx: &mut AnyTransaction,
    id: Id,
    new_email: &str,
) -> Result<User, UserError> {
    tx.update_user_email_by_id(id, new_email)
        .await
        .map_err(UserError::from)?
        .ok_or(UserError::UserNotFound)
        .map(User::from)
}

/// Updates a user's password by their identifier.
#[instrument(err, skip(tx, new_password))]
pub async fn update_user_password_by_id(
    tx: &mut AnyTransaction,
    id: Id,
    new_password: Option<&str>,
) -> Result<User, UserError> {
    tx.update_user_password_by_id(id, new_password)
        .await
        .map_err(UserError::from)?
        .ok_or(UserError::UserNotFound)
        .map(User::from)
}

impl From<database::users::User> for User {
    fn from(value: database::users::User) -> Self {
        Self {
            id: value.id,
            email: value.email,
            has_password: value.password.is_some(),
        }
    }
}

impl From<database::users::User> for UserWithPassword {
    fn from(value: database::users::User) -> Self {
        Self {
            id: value.id,
            email: value.email,
            password: value.password,
        }
    }
}

impl From<UserWithPassword> for User {
    fn from(value: UserWithPassword) -> Self {
        Self {
            id: value.id,
            email: value.email,
            has_password: value.password.is_some(),
        }
    }
}

/// An error returned by all user operations.
#[derive(Debug, PartialEq)]
pub enum UserError {
    Database(database::Error),
    UserNotFound,
}

impl std::error::Error for UserError {}
impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<database::Error> for UserError {
    fn from(value: database::Error) -> Self {
        Self::Database(value)
    }
}
