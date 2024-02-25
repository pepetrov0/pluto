use axum::{extract::State, response::Redirect, routing, Router};

use crate::{
    core::database::WriteRepository,
    domain::{self, sessions::Session},
    AppState,
};

use super::session_providers::cookie::RemoveCookieSession;

async fn handler(
    State(state): State<AppState>,
    session: Option<Session>,
) -> (RemoveCookieSession, Redirect) {
    let mut repository = match WriteRepository::from_pool(&state.database).await {
        Some(tx) => tx,
        None => return (RemoveCookieSession, Redirect::to("/")),
    };

    if let Some(session) = session {
        let _ = domain::sessions::delete(&mut repository, &session).await;
    }

    let _ = repository.commit().await;
    (RemoveCookieSession, Redirect::to("/"))
}

pub fn router() -> Router<AppState> {
    Router::new().route("/logout", routing::any(handler))
}
