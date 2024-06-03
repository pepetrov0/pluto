use axum::{extract::State, response::Response, Form};
use axum_extra::{headers::UserAgent, TypedHeader};
use secrecy::Secret;

use crate::{
    domain::{
        authentication::{self, AuthenticationError},
        database::{Database, Transaction},
    },
    web::{
        _components::organisms::LoginData,
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
    Form(data): Form<LoginData>,
) -> Response {
    let respond = |r| responder::invoke(locale.clone(), hx, r);

    // first attempt creating a transaction
    let mut transaction = match state.database.begin().await {
        Ok(t) => t,
        Err(_) => return respond(Err(AuthenticationError::Failure)).await,
    };

    // then attempt registering the user
    let (_, result) = match authentication::authenticate(
        &mut transaction,
        &data.email,
        &Secret::from(data.password.clone()),
        agent.0.as_str(),
    )
    .await
    {
        Ok(r) => r,
        Err(e) => return respond(Err(e)).await,
    };

    // then attempt committing the transaction
    if transaction.commit().await.is_err() {
        return respond(Err(AuthenticationError::Failure)).await;
    }

    // invoke the responder
    respond(Ok(result)).await
}
