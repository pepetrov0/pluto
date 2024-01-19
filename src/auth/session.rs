//! Implements a session component

use axum::{async_trait, extract::FromRequestParts, Extension, http::request::Parts};
use nanoid::nanoid;
use sqlx::prelude::FromRow;

use crate::{imkvs::InMemoryKeyValueStore, AppState};

/// Represents a session
#[derive(Debug, Clone, FromRow)]
pub struct Session {
    /// The session id
    pub id: String,
    /// A user id or email
    pub user: String,
}

/// A session repository
#[async_trait]
pub trait SessionRepository: Send + Sync {
    /// Creates a session
    async fn create_session(&self, user: String) -> Option<Session>;

    /// Finds a session
    async fn find_session(&self, id: &str) -> Option<Session>;

    /// Deletes a session
    async fn delete_session(&self, id: &str);
}

#[async_trait]
impl SessionRepository for InMemoryKeyValueStore<String, Session> {
    async fn create_session(&self, user: String) -> Option<Session> {
        let mut map = self.lock().await;
        let session = Session {
            id: nanoid!(),
            user,
        };

        map.insert(session.id.clone(), session.clone());
        Some(session)
    }

    async fn find_session(&self, id: &str) -> Option<Session> {
        let map = self.lock().await;
        map.get(id).cloned()
    }

    async fn delete_session(&self, id: &str) {
        let mut map = self.lock().await;
        map.remove(id);
    }
}

#[async_trait]
impl FromRequestParts<AppState> for Session {
    type Rejection = <Extension<Session> as FromRequestParts<AppState>>::Rejection;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection>{
        Extension::from_request_parts(parts, state).await.map(|v| v.0)
    }
}