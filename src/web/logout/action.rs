use axum::{extract::State, response::Response};

use crate::{
    domain::{
        database::{Database, Transaction},
        logout::logout,
    },
    web::_core::{Auth, GlobalState},
};

#[tracing::instrument(skip(state))]
pub async fn invoke(State(state): State<GlobalState>, auth: Option<Auth>) -> Response {
    if let Some(session) = auth.and_then(|v| v.session) {
        if let Ok(mut tx) = state.database.begin().await {
            let _ = logout(&mut tx, &session).await;
            let _ = tx.commit().await;
        }
    }

    super::responder::invoke().await
}
