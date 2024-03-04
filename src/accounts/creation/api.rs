use axum::{extract::State, response::Redirect, Form};
use serde::Deserialize;

use crate::{
    auth::principal::AuthPrincipal,
    core::database::WriteRepository,
    domain::{self, csrf_tokens},
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
    State(state): State<AppState>,
    Form(details): Form<NewAccountForm>,
) -> Result<Redirect, AccountCreationError> {
    let details = NewAccountForm {
        name: details.name.trim().to_owned(),
        csrf: details.csrf.trim().to_owned(),
    };

    let mut repository = WriteRepository::from_pool(&state.database)
        .await
        .ok_or(AccountCreationError::Unknown)?;

    // check csrf
    if !csrf_tokens::verify(
        &mut repository,
        &details.csrf,
        &user,
        super::CSRF_TOKEN_USAGE,
    )
    .await
    .unwrap_or(false)
    {
        return Err(AccountCreationError::InvalidCsrf);
    }

    // create account and ownership
    let account = domain::accounts::create(&mut repository, &details.name)
        .await
        .map_err(AccountCreationError::from)?;
    let _ = domain::accounts_ownerships::create(&mut repository, &user, &account)
        .await
        .map_err(AccountCreationError::from)?;

    repository
        .commit()
        .await
        .ok_or(AccountCreationError::Unknown)?;
    Ok(Redirect::to("/accounts?created=true"))
}

impl From<domain::accounts::AccountCreationError> for AccountCreationError {
    fn from(value: domain::accounts::AccountCreationError) -> Self {
        match value {
            domain::accounts::AccountCreationError::Unknown => Self::Unknown,
            domain::accounts::AccountCreationError::InvalidName => Self::InvalidName,
        }
    }
}
