use axum::{extract::State, response::Response};

use crate::{
    domain::{
        database::{Database, Transaction},
        sessions::delete_session_by_id,
    },
    web::_core::{Auth, GlobalState},
};

pub async fn invoke(State(state): State<GlobalState>, auth: Option<Auth>) -> Response {
    if let Some(session) = auth.and_then(|v| v.session) {
        if let Ok(mut tx) = state.database.begin().await {
            let _ = delete_session_by_id(&mut tx, session.id).await;
            let _ = tx.commit().await;
        }
    }

    super::responder::invoke().await
}
