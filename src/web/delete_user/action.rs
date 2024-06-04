use axum::{extract::State, response::Response, Form};

use crate::{
    domain::{
        database::{Database, Transaction},
        delete_user::{delete_user, DeleteUserError},
    },
    web::{
        _components::organisms::DangerZoneData,
        _core::{Auth, GlobalState, Hx, Locale},
    },
};

pub async fn invoke(
    State(state): State<GlobalState>,
    locale: Locale,
    hx: Hx,
    auth: Auth,
    Form(data): Form<DangerZoneData>,
) -> Response {
    let respond = |e| super::responder::invoke(locale, hx, &auth.user, e);

    let mut tx = match state.database.begin().await {
        Ok(tx) => tx,
        Err(_) => return respond(Some(DeleteUserError::Failure)).await,
    };

    if let Err(e) = delete_user(&mut tx, &auth.user, &data.password).await {
        return respond(Some(e)).await;
    }

    match tx.commit().await.is_ok() {
        true => respond(None).await,
        false => return respond(Some(DeleteUserError::Failure)).await,
    }
}
