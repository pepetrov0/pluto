use axum::{extract::State, response::Response, Form};
use secrecy::Secret;

use crate::{
    domain::{
        database::Database,
        registration::{self, RegistrationError},
    },
    web::{
        _components::organisms::RegisterFormData,
        _core::{GlobalState, Locale},
    },
};

#[tracing::instrument(skip(state))]
pub async fn invoke(
    State(state): State<GlobalState>,
    locale: Locale,
    Form(data): Form<RegisterFormData>,
) -> Response {
    // first attempt creating a transaction
    let mut transaction = match state.database.begin().await {
        Ok(t) => t,
        Err(_) => {
            return super::responder::invoke(locale, data, Some(RegistrationError::Failure)).await
        }
    };

    // then attempt registering the user
    let error = registration::validate_register(
        &mut transaction,
        &data.email,
        &Secret::from(data.password.clone()),
        &Secret::from(data.confirm_password.clone()),
    )
    .await
    .err();

    // invoke the responder
    super::responder::invoke(locale, data, error).await
}
