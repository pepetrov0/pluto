//! Implements CSRF tokens

use axum::async_trait;
use chrono::{NaiveDateTime, Utc};

use crate::imkvs::InMemoryKeyValueStore;

pub const CSRF_TOKEN_LIFETIME: i64 = 1800; // in seconds

/// Represents a CSRF token
#[derive(Debug, Clone)]
pub struct CsrfToken {
    pub id: String,
    pub user: String,
    pub usage: String,
    pub created_at: NaiveDateTime,
}

/// Represents a CSRF token repository
#[async_trait]
pub trait CsrfTokenRepository: Send + Sync {
    /// Find and remove a CSRF token
    async fn consume_csrf_token(&self, id: &str) -> Option<CsrfToken>;

    /// Create a CSRF token
    async fn create_csrf_token(&self, user: String, usage: &str) -> Option<CsrfToken>;
}

#[async_trait]
impl CsrfTokenRepository for InMemoryKeyValueStore<String, CsrfToken> {
    #[tracing::instrument]
    async fn consume_csrf_token(&self, id: &str) -> Option<CsrfToken> {
        let mut map = self.lock().await;
        let now = Utc::now().naive_utc();

        // cleanup expired tokens
        map.retain(|_, v| (now - v.created_at).num_seconds() < CSRF_TOKEN_LIFETIME);

        map.remove(id)
    }

    #[tracing::instrument]
    async fn create_csrf_token(&self, user: String, usage: &str) -> Option<CsrfToken> {
        let mut map = self.lock().await;
        let now = Utc::now().naive_utc();

        // cleanup expired tokens
        map.retain(|_, v| (now - v.created_at).num_seconds() < CSRF_TOKEN_LIFETIME);

        // construct a new csrf token
        let token = CsrfToken {
            id: nanoid::nanoid!(),
            user,
            usage: usage.to_owned(),
            created_at: now,
        };

        // save the new token
        map.insert(token.id.clone(), token.clone());

        Some(token)
    }
}
