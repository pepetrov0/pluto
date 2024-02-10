//! Implements the account creation API

use axum::{extract::State, response::Redirect, Form};
use serde::Deserialize;

use crate::{
    accounts::{component::AccountRepository, ownership::AccountOwnershipRepository},
    auth::principal::AuthPrincipal,
    csrf_tokens::CsrfTokenRepository,
    AppState,
};

use super::error::AccountCreationError;

#[derive(Debug, Clone, Deserialize)]
pub struct NewAccountForm {
    pub name: String,
    pub csrf: String,
}

pub async fn handler(
    AuthPrincipal(user): AuthPrincipal,
    State(mut state): State<AppState>,
    Form(details): Form<NewAccountForm>,
) -> Result<Redirect, AccountCreationError> {
    let details = NewAccountForm {
        name: details.name.trim().to_owned(),
        csrf: details.csrf.trim().to_owned(),
    };

    // check csrf
    let csrf = state
        .database
        .consume_csrf_token(details.csrf.as_str())
        .await;
    if csrf
        .filter(|v| v.usr == user.id)
        .filter(|v| v.usage == super::CSRF_TOKEN_USAGE)
        .is_none()
    {
        return Err(AccountCreationError::InvalidCsrf);
    }

    // check for a missing name
    if details.name.is_empty() || details.name.len() > 200 {
        return Err(AccountCreationError::InvalidName);
    }

    // create account and ownership
    let account = state
        .database
        .create_account(details.name)
        .await
        .ok_or(AccountCreationError::Unknown)?;
    let _ = state
        .database
        .create_account_ownership(user.id, account.id)
        .await
        .ok_or(AccountCreationError::Unknown)?;

    Ok(Redirect::to("/accounts?created=true"))
}
