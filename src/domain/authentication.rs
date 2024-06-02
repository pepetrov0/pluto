use tracing::instrument;

use super::{
    database::AnyTransaction,
    passwords,
    sessions::{Session, SessionError, SessionsRepository},
    users::{User, UserError, UsersRepository},
};

#[derive(Debug, Clone)]
pub enum AuthenticationError {
    InvalidCredentials,
    Failure,
}

/// Authenticates a user by their email, password and agent.
/// Returns the user authenticated as well as a new session.
#[instrument(err, skip(tx, password))]
pub async fn authenticate(
    tx: &mut AnyTransaction,
    email: &str,
    password: &str,
    agent: &str,
) -> Result<(User, Session), AuthenticationError> {
    let user = tx
        .find_user_by_email(email)
        .await
        .map_err(AuthenticationError::from)?;

    // validate password
    user.password
        .as_ref()
        .and_then(|v| passwords::verify_password(password, v.as_str()).ok())
        .ok_or(AuthenticationError::InvalidCredentials)?;

    // create session
    let session = tx
        .create_session(user.id, agent)
        .await
        .map_err(AuthenticationError::from)?;

    Ok((user, session))
}

impl std::error::Error for AuthenticationError {}
impl std::fmt::Display for AuthenticationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<UserError> for AuthenticationError {
    fn from(value: UserError) -> Self {
        match value {
            UserError::Message(_) => Self::Failure,
            UserError::NotFound => Self::InvalidCredentials,
        }
    }
}

impl From<SessionError> for AuthenticationError {
    fn from(value: SessionError) -> Self {
        match value {
            SessionError::Message(_) | SessionError::NotFound => Self::Failure,
        }
    }
}
