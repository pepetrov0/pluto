//! Implements the account creation API

use axum::{extract::State, response::Redirect, Form};
use chrono::Utc;
use serde::Deserialize;

use crate::{auth::principal::AuthPrincipal, csrf_tokens, AppState};

use super::error::AccountCreationError;

#[derive(Debug, Clone, Deserialize)]
pub struct NewAccountForm {
    pub name: String,
    pub default_asset: Option<String>,
    pub csrf: String,
}

pub async fn handler(
    AuthPrincipal(user): AuthPrincipal,
    State(state): State<AppState>,
    Form(details): Form<NewAccountForm>,
) -> Result<Redirect, AccountCreationError> {
    let details = NewAccountForm {
        name: details.name.trim().to_owned(),
        default_asset: details
            .default_asset
            .map(|v| v.trim().to_owned())
            .filter(|v| !v.is_empty()),
        csrf: details.csrf.trim().to_owned(),
    };

    // check csrf
    let csrf = state
        .csrf_token_repository
        .consume_csrf_token(details.csrf.as_str())
        .await;
    if csrf
        .filter(|v| v.user == user.id)
        .filter(|v| v.usage == super::CSRF_TOKEN_USAGE)
        .filter(|v| {
            (Utc::now().naive_utc() - v.created_at).num_seconds() < csrf_tokens::CSRF_TOKEN_LIFETIME
        })
        .is_none()
    {
        return Err(AccountCreationError::InvalidCsrf);
    }

    // check for a missing name
    if details.name.is_empty() {
        return Err(AccountCreationError::MissingName);
    }

    // create account and ownership
    let account = state
        .account_repository
        .create_account(details.name, details.default_asset)
        .await
        .ok_or(AccountCreationError::Unknown)?;
    let _ = state
        .account_ownership_repository
        .create_account_ownership(user.id, account.id)
        .await
        .ok_or(AccountCreationError::Unknown)?;

    Ok(Redirect::to("/accounts?created=true"))
}
