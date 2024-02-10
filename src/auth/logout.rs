//! Implements a logout endpoint

use axum::{extract::State, response::Redirect, routing, Router};

use crate::AppState;

use super::{
    session::{Session, SessionRepository},
    session_providers::cookie::RemoveCookieSession,
};

async fn handler(
    State(mut state): State<AppState>,
    session: Option<Session>,
) -> (RemoveCookieSession, Redirect) {
    if let Some(session) = session {
        state.database.delete_session(&session.id).await;
    }

    (RemoveCookieSession, Redirect::to("/"))
}

pub fn router() -> Router<AppState> {
    Router::new().route("/logout", routing::any(handler))
}
