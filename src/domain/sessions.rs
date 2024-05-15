//! This module implements sessions business logic.

use chrono::NaiveDateTime;

use super::{
    database::{self, sessions::Sessions, AnyTransaction},
    identifier::Id,
};

/// A session.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Session {
    pub id: Id,
    pub user_id: Id,
    pub agent: String,
    pub created_on: NaiveDateTime,
}

impl From<database::sessions::Session> for Session {
    fn from(value: database::sessions::Session) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            agent: value.agent,
            created_on: value.created_on,
        }
    }
}

/// An error related to session operations.
pub enum SessionError {
    Database(database::Error),
    SessionNotFound,
}

impl From<database::Error> for SessionError {
    fn from(value: database::Error) -> Self {
        Self::Database(value)
    }
}

/// Finds a session by its identifier.
pub async fn find_session_by_id(
    transaction: &mut AnyTransaction,
    id: Id,
) -> Result<Session, SessionError> {
    transaction
        .find_session_by_id(id)
        .await
        .map_err(SessionError::from)?
        .ok_or(SessionError::SessionNotFound)
        .map(Session::from)
}
