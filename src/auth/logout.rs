use axum::{routing, Router, extract::State, response::Redirect};

use crate::AppState;

use super::{session::Session, session_providers::cookie::RemoveCookieSession};

async fn handler(State(state): State<AppState>, session: Option<Session>) -> (RemoveCookieSession, Redirect) {
    if let Some(session) = session {
        state.session_repository.delete_session(&session.id).await;
    }

    (RemoveCookieSession, Redirect::to("/"))
}

pub fn router() -> Router<AppState> {
    Router::new().route("/logout", routing::any(handler))
}
