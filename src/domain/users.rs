//! This module implements the service layer for managing users.

use super::{
    database::{self, users::Users, AnyTransaction},
    identifier::Id,
};

/// A user.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct User {
    pub id: Id,
    pub email: String,
}

impl From<database::users::User> for User {
    fn from(value: database::users::User) -> Self {
        Self {
            id: value.id,
            email: value.email,
        }
    }
}

/// An error returned by all user operations.
pub enum UserError {
    Database(database::Error),
    UserNotFound,
}

impl From<database::Error> for UserError {
    fn from(value: database::Error) -> Self {
        Self::Database(value)
    }
}

/// Finds a user by their identifier.
pub async fn find_user_by_id(transaction: &mut AnyTransaction, id: Id) -> Result<User, UserError> {
    transaction
        .find_user_by_id(id)
        .await
        .map_err(UserError::from)?
        .ok_or(UserError::UserNotFound)
        .map(User::from)
}
