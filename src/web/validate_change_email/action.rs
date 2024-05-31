use axum::{extract::State, response::Response, Form};

use crate::{
    domain::{
        change_email::{validate_change_email, ChangeEmailError},
        database::Database,
    },
    web::{
        _components::organisms::ChangeEmailFormData,
        _core::{Auth, GlobalState, Locale},
    },
};

pub async fn invoke(
    State(state): State<GlobalState>,
    locale: Locale,
    auth: Auth,
    Form(data): Form<ChangeEmailFormData>,
) -> Response {
    let respond = |e| super::responder::invoke(locale, data.clone(), e);

    let mut tx = match state.database.begin().await {
        Ok(tx) => tx,
        Err(_) => return respond(Some(ChangeEmailError::Failure)).await,
    };

    let error = validate_change_email(&mut tx, &auth.user, data.new_email.as_str())
        .await
        .err();
    respond(error).await
}