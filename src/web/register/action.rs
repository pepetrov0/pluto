use axum::{extract::State, response::Response, Form};
use axum_extra::{headers::UserAgent, TypedHeader};

use crate::{
    domain::{
        database::{Database, Transaction},
        registration::{self, RegistrationError},
    },
    web::_core::{GlobalState, Hx, Locale},
};

#[tracing::instrument(skip(state, args))]
pub async fn invoke(
    State(state): State<GlobalState>,
    locale: Locale,
    hx: Hx,
    agent: TypedHeader<UserAgent>,
    Form(args): Form<super::Arguments>,
) -> Response {
    use super::responder;

    // first attempt creating a transaction
    let mut transaction = match state.database.begin().await {
        Ok(t) => t,
        Err(_) => {
            return responder::invoke(locale, hx, state.key, args, Err(RegistrationError::Failure))
                .await
        }
    };

    // then attempt registering the user
    let result = registration::register_and_authenticate(
        &mut transaction,
        &args.email,
        &args.password,
        &args.confirm_password,
        agent.0.as_str(),
    )
    .await;

    // then attempt committing the transaction
    if transaction.commit().await.is_err() {
        return responder::invoke(locale, hx, state.key, args, Err(RegistrationError::Failure))
            .await;
    }

    // invoke the responder
    responder::invoke(locale, hx, state.key, args, result).await
}
