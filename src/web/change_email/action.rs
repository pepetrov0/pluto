use axum::{extract::State, response::Response, Form};

use crate::{
    domain::{
        change_email::{change_email, ChangeEmailError},
        database::{Database, Transaction},
    },
    web::{
        _components::organisms::ChangeEmailFormData,
        _core::{Auth, GlobalState, Hx, Locale},
    },
};

pub async fn invoke(
    State(state): State<GlobalState>,
    locale: Locale,
    hx: Hx,
    auth: Auth,
    Form(data): Form<ChangeEmailFormData>,
) -> Response {
    let respond = |e| super::responder::invoke(locale, hx, &auth.user, data.clone(), e);

    let mut tx = match state.database.begin().await {
        Ok(tx) => tx,
        Err(_) => return respond(Some(ChangeEmailError::Failure)).await,
    };

    if let Err(error) =
        change_email(&mut tx, &auth.user, &data.new_email, &data.current_password).await
    {
        return respond(Some(error)).await;
    }

    match tx.commit().await.is_ok() {
        true => respond(None).await,
        false => respond(Some(ChangeEmailError::Failure)).await,
    }
}
