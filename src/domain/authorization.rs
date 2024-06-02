//! Implements authorization.

use tracing::instrument;

use super::{
    database::AnyTransaction,
    sessions::{SessionError, SessionsRepository},
    users::{UserError, UsersRepository},
    Id, Session, User,
};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum AuthorizationError {
    UserNotFound,
    SessionNotFound,
    AgentMismatch,
    Failure,
}

/// Force authorizes a user by email.
/// If the user does not exist, it is created automatically.
#[instrument(err, skip(tx))]
pub async fn force_authorize_by_email(
    tx: &mut AnyTransaction,
    email: &str,
) -> Result<User, AuthorizationError> {
    let user = match tx.find_user_by_email(email).await {
        Ok(user) => user,
        Err(UserError::NotFound) => tx
            .create_user(email, None)
            .await
            .map_err(AuthorizationError::from)?,
        Err(e) => return Err(AuthorizationError::from(e)),
    };

    Ok(user)
}

/// Authorizes a user by session.
#[instrument(err, skip(tx))]
pub async fn authorize_by_session(
    tx: &mut AnyTransaction,
    id: Id,
    agent: &str,
) -> Result<(User, Session), AuthorizationError> {
    let session = tx
        .find_session_by_id(id)
        .await
        .map_err(AuthorizationError::from)?;

    // check agent
    if session.agent != agent {
        return Err(AuthorizationError::AgentMismatch);
    }

    let user = tx
        .find_user_by_id(session.user_id)
        .await
        .map_err(AuthorizationError::from)?;

    Ok((user, session))
}

impl std::error::Error for AuthorizationError {}
impl std::fmt::Display for AuthorizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<UserError> for AuthorizationError {
    fn from(value: UserError) -> Self {
        match value {
            UserError::Message(_) => Self::Failure,
            UserError::NotFound => Self::UserNotFound,
        }
    }
}

impl From<SessionError> for AuthorizationError {
    fn from(value: SessionError) -> Self {
        match value {
            SessionError::Message(_) => Self::Failure,
            SessionError::NotFound => Self::SessionNotFound,
        }
    }
}
