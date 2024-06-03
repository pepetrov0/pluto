use axum::{extract::State, response::Response, Form};
use secrecy::Secret;

use crate::{
    domain::{
        change_password::{change_password, ChangePasswordError},
        database::{Database, Transaction},
    },
    web::{
        _components::organisms::ChangePasswordData,
        _core::{Auth, GlobalState, Hx, Locale},
    },
};

#[tracing::instrument(skip(state))]
pub async fn invoke(
    State(state): State<GlobalState>,
    locale: Locale,
    hx: Hx,
    auth: Auth,
    Form(data): Form<ChangePasswordData>,
) -> Response {
    let respond = |e| super::responder::invoke(locale, hx, &auth.user, data.clone(), e);

    let mut tx = match state.database.begin().await {
        Ok(tx) => tx,
        Err(_) => return respond(Some(ChangePasswordError::Failure)).await,
    };

    if let Err(error) = change_password(
        &mut tx,
        &auth.user,
        &Secret::from(data.new_password.clone()),
        &Secret::from(data.confirm_new_password.clone()),
        &Secret::from(data.current_password.clone()),
    )
    .await
    {
        return respond(Some(error)).await;
    }

    match tx.commit().await.is_ok() {
        true => respond(None).await,
        false => respond(Some(ChangePasswordError::Failure)).await,
    }
}
