//! This module implements sessions business logic.

use axum::async_trait;
use chrono::NaiveDateTime;

use crate::domain::identifier::Id;

/// An entity representing a session.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Session {
    /// Identifier of the session.
    pub id: Id,
    /// Identifier to the related user.
    pub user_id: Id,
    /// Agent of the user.
    pub agent: String,
    /// The timestamp of the creation of the session.
    pub created_on: NaiveDateTime,
}

/// An error that might occur while working with sessions.
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum SessionError {
    /// A general error described by a message.
    Message(String),
    /// An error indicating that no row was found by the query.
    NotFound,
}

/// A trait describing a repository of sessions.
#[async_trait]
pub trait SessionsRepository {
    /// Finds a session by identifier.
    async fn find_session_by_id(&mut self, id: Id) -> Result<Session, SessionError>;

    /// Finds all sessions for the specified user.
    async fn find_all_sessions_by_user_id(
        &mut self,
        user_id: Id,
    ) -> Result<Vec<Session>, SessionError>;

    /// Create a session.
    async fn create_session(&mut self, user_id: Id, agent: &str) -> Result<Session, SessionError>;

    /// Deletes a session by ID.
    async fn delete_session_by_id(&mut self, id: Id) -> Result<(), SessionError>;

    /// Deletes all sessions of the specified user.
    async fn delete_all_sessions_by_user_id(&mut self, user_id: Id) -> Result<(), SessionError>;
}

impl std::error::Error for SessionError {}
impl std::fmt::Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<sqlx::Error> for SessionError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Self::NotFound,
            e => Self::Message(format!("{e:?}")),
        }
    }
}
