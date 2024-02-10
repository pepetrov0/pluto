//! Implements a logout endpoint

use axum::{extract::State, response::Redirect, routing, Router};

use crate::AppState;

use super::{
    session::{Session, SessionWriteRepository},
    session_providers::cookie::RemoveCookieSession,
};

async fn handler(
    State(state): State<AppState>,
    session: Option<Session>,
) -> (RemoveCookieSession, Redirect) {
    let mut tx = match state.database.begin().await {
        Ok(tx) => tx,
        Err(_) => return (RemoveCookieSession, Redirect::to("/")),
    };

    if let Some(session) = session {
        tx.delete_session(&session.id).await;
    }

    let _ = tx.commit().await;
    (RemoveCookieSession, Redirect::to("/"))
}

pub fn router() -> Router<AppState> {
    Router::new().route("/logout", routing::any(handler))
}
