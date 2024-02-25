use axum::{extract::State, response::Redirect, Form};
use serde::Deserialize;

use crate::{
    accounts::{component::AccountWriteRepository, ownership::AccountOwnershipWriteRepository},
    auth::principal::AuthPrincipal,
    core::database::WriteRepository,
    domain::csrf_tokens,
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
    {
        return Err(AccountCreationError::InvalidCsrf);
    }

    // check for a missing name
    if details.name.is_empty() || details.name.len() > 200 {
        return Err(AccountCreationError::InvalidName);
    }

    // create account and ownership
    let account = repository
        .create_account(&details.name)
        .await
        .ok_or(AccountCreationError::Unknown)?;
    let _ = repository
        .create_account_ownership(&user.id, &account.id)
        .await
        .ok_or(AccountCreationError::Unknown)?;

    repository
        .commit()
        .await
        .ok_or(AccountCreationError::Unknown)?;
    Ok(Redirect::to("/accounts?created=true"))
}
