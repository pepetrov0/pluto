//! This module implements sessions business logic.

use chrono::NaiveDateTime;
use tracing::instrument;

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
#[derive(Debug)]
pub enum SessionError {
    Database(database::Error),
    SessionNotFound,
}

impl std::error::Error for SessionError {}
impl std::fmt::Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<database::Error> for SessionError {
    fn from(value: database::Error) -> Self {
        Self::Database(value)
    }
}

/// Finds a session by its identifier.
#[instrument(err, skip(tx))]
pub async fn find_session_by_id(tx: &mut AnyTransaction, id: Id) -> Result<Session, SessionError> {
    tx.find_session_by_id(id)
        .await
        .map_err(SessionError::from)?
        .ok_or(SessionError::SessionNotFound)
        .map(Session::from)
}

/// Creates a session.
#[instrument(err, skip(tx))]
pub async fn create_session(
    tx: &mut AnyTransaction,
    user: Id,
    agent: &str,
) -> Result<Session, SessionError> {
    tx.create_session(user, agent)
        .await
        .map(Session::from)
        .map_err(SessionError::from)
}

/// Deletes a session by its identifier.
#[instrument(err, skip(tx))]
pub async fn delete_session_by_id(tx: &mut AnyTransaction, id: Id) -> Result<(), SessionError> {
    tx.delete_session_by_id(id)
        .await
        .map_err(SessionError::from)
}
