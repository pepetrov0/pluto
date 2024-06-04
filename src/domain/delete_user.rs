//! A service implementation to delete a user.

use secrecy::{ExposeSecret, Secret};
use tracing::instrument;

use super::{
    database::AnyTransaction,
    passwords,
    users::{UserError, UsersRepository},
    User,
};

/// An error that might occur while deleting a user.
#[derive(Debug, Clone, Copy)]
pub enum DeleteUserError {
    Failure,
    InvalidCredentials,
}

/// Deletes a user by their identifier.
#[instrument(err, skip(tx))]
pub async fn delete_user(
    tx: &mut AnyTransaction,
    user: &User,
    password: &Secret<String>,
) -> Result<(), DeleteUserError> {
    let password = password.expose_secret().as_str();

    // validate password
    user.password
        .as_ref()
        .and_then(|v| passwords::verify_password(password, v.expose_secret().as_str()).ok())
        .ok_or(DeleteUserError::InvalidCredentials)?;

    // attempt deleting a user
    tx.delete_user_by_id(user.id)
        .await
        .map_err(DeleteUserError::from)
}

impl std::error::Error for DeleteUserError {}
impl std::fmt::Display for DeleteUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<UserError> for DeleteUserError {
    fn from(value: UserError) -> Self {
        match value {
            UserError::Message(_) | UserError::NotFound => Self::Failure,
        }
    }
}
