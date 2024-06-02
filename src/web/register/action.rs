use axum::{extract::State, response::Response, Form};
use axum_extra::{headers::UserAgent, TypedHeader};
use secrecy::Secret;

use crate::{
    domain::{
        database::{Database, Transaction},
        registration::{self, RegistrationError},
    },
    web::{
        _components::organisms::RegisterFormData,
        _core::{GlobalState, Hx, Locale},
    },
};

use super::responder;

#[tracing::instrument(skip(state))]
pub async fn invoke(
    State(state): State<GlobalState>,
    locale: Locale,
    hx: Hx,
    agent: TypedHeader<UserAgent>,
    Form(data): Form<RegisterFormData>,
) -> Response {
    let respond = |r| responder::invoke(locale.clone(), hx, data.clone(), r);

    // first attempt creating a transaction
    let mut transaction = match state.database.begin().await {
        Ok(t) => t,
        Err(_) => return respond(Err(RegistrationError::Failure)).await,
    };

    // then attempt registering the user
    let (_, result) = match registration::register_and_authenticate(
        &mut transaction,
        &data.email,
        &Secret::from(data.password.clone()),
        &Secret::from(data.confirm_password.clone()),
        agent.0.as_str(),
    )
    .await
    {
        Ok(r) => r,
        Err(e) => return respond(Err(e)).await,
    };

    // then attempt committing the transaction
    if transaction.commit().await.is_err() {
        return respond(Err(RegistrationError::Failure)).await;
    }

    // invoke the responder
    respond(Ok(result)).await
}
