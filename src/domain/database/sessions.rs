//! This module implements the concept of a session.

use axum::async_trait;
use chrono::NaiveDateTime;

use crate::domain::identifier::Id;

/// An entity representing a session.
#[derive(Debug, Clone, sqlx::FromRow, Eq, PartialEq)]
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

/// A trait describing a repository of sessions.
#[async_trait]
pub trait Sessions {
    /// Finds a session by identifier.
    async fn find_session_by_id(&mut self, id: Id) -> super::Result<Option<Session>>;

    /// Finds all sessions for the specified user.
    async fn find_all_sessions_by_user_id(&mut self, user_id: Id) -> super::Result<Vec<Session>>;

    /// Create a session.
    async fn create_session(&mut self, user_id: Id, agent: &str) -> super::Result<Session>;

    /// Deletes a session by ID.
    async fn delete_session_by_id(&mut self, id: Id) -> super::Result<()>;

    /// Deletes all sessions of the specified user.
    async fn delete_all_sessions_by_user_id(&mut self, user_id: Id) -> super::Result<()>;
}
