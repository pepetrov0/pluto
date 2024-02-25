//! Implements the API relevant for user registration

use axum::{extract::State, response::Redirect, Form};
use chrono_tz::Tz;

use crate::{
    accounts::{component::AccountWriteRepository, ownership::AccountOwnershipWriteRepository},
    auth::{
        principal::NoAuthPrincipal, session::SessionWriteRepository,
        session_providers::cookie::SetCookieSession,
    },
    database::WriteRepository,
    domain, validation, AppState,
};

use super::error::RegistrationError;

const DEFAULT_ACCOUNT_NAME: &str = "Default";

#[derive(serde::Deserialize)]
pub struct RegisterForm {
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub favorite_asset: String,
    pub timezone: String,
}

pub async fn handler(
    _: NoAuthPrincipal,
    State(state): State<AppState>,
    Form(details): Form<RegisterForm>,
) -> Result<(SetCookieSession, Redirect), RegistrationError> {
    let mut repository = WriteRepository::from_pool(&state.database)
        .await
        .ok_or(RegistrationError::Unknown)?;

    // trim details (email)
    let details = RegisterForm {
        email: details.email.trim().to_owned(),
        ..details
    };

    // validate email
    if !validation::is_email(&details.email) {
        return Err(RegistrationError::InvalidEmail);
    }

    // validate matching passwords
    if details.password != details.confirm_password {
        return Err(RegistrationError::PasswordsMismatch);
    }

    // attempt hashing the password
    let hash = match state.password_hasher.hash(details.password.as_bytes()) {
        Some(hash) => hash,
        None => return Err(RegistrationError::Unknown),
    };

    // find asset
    let favorite_asset =
        domain::assets::find_by_id_or_ticker(&mut repository, &details.favorite_asset)
            .await
            .ok_or(RegistrationError::InvalidFavoriteAsset)?;

    // parse timezone
    let timezone = Tz::from_str_insensitive(&details.timezone).unwrap_or_default();

    // create default account
    let favorite_account = repository
        .create_account(DEFAULT_ACCOUNT_NAME)
        .await
        .ok_or(RegistrationError::Unknown)?;

    // attempt creating a user
    let user = domain::users::create(
        &mut repository,
        &details.email,
        Some(hash),
        timezone,
        &favorite_asset,
        &favorite_account,
    )
    .await
    .map_err(RegistrationError::from)?;

    // create ownership to default account
    repository
        .create_account_ownership(&user.id, &favorite_account.id)
        .await
        .ok_or(RegistrationError::Unknown)?;

    // create a session
    let session = repository
        .create_session(user.id)
        .await
        .ok_or(RegistrationError::Unknown)?;

    repository
        .commit()
        .await
        .ok_or(RegistrationError::Unknown)?;
    Ok((SetCookieSession(session), Redirect::to("/")))
}
