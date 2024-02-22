//! Implements a logout endpoint

use axum::{extract::State, response::Redirect, routing, Router};

use crate::{database::WriteRepository, AppState};

use super::{
    session::{Session, SessionWriteRepository},
    session_providers::cookie::RemoveCookieSession,
};

async fn handler(
    State(state): State<AppState>,
    session: Option<Session>,
) -> (RemoveCookieSession, Redirect) {
    let mut repository = match WriteRepository::from_pool(&state.database).await {
        Some(tx) => tx,
        None => return (RemoveCookieSession, Redirect::to("/")),
    };

    if let Some(session) = session {
        repository.delete_session(&session.id).await;
    }

    let _ = repository.commit().await;
    (RemoveCookieSession, Redirect::to("/"))
}

pub fn router() -> Router<AppState> {
    Router::new().route("/logout", routing::any(handler))
}
