//! Implements the API relevant for user registration

use axum::{extract::State, response::Redirect, Form};
use chrono_tz::Tz;

use crate::{
    accounts::{component::AccountWriteRepository, ownership::AccountOwnershipWriteRepository},
    assets::component::AssetReadonlyRepository,
    auth::{
        principal::NoAuthPrincipal, session::SessionWriteRepository,
        session_providers::cookie::SetCookieSession,
    },
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
    let mut tx = state
        .database
        .begin()
        .await
        .map_err(|_| RegistrationError::Unknown)?;

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
    let favorite_asset = tx
        .find_asset(&details.favorite_asset)
        .await
        .ok_or(RegistrationError::InvalidFavoriteAsset)?;

    // parse timezone
    let timezone = Tz::from_str_insensitive(&details.timezone).unwrap_or_default();

    // create default account
    let favorite_account = tx
        .create_account(DEFAULT_ACCOUNT_NAME)
        .await
        .ok_or(RegistrationError::Unknown)?;

    // attempt creating a user
    let user = domain::users::create(
        &mut tx,
        &details.email,
        Some(hash),
        timezone,
        &favorite_asset,
        &favorite_account,
    )
    .await
    .map_err(RegistrationError::from)?;

    // create ownership to default account
    tx.create_account_ownership(&user.id, &favorite_account.id)
        .await
        .ok_or(RegistrationError::Unknown)?;

    // create a session
    let session = tx
        .create_session(user.id)
        .await
        .ok_or(RegistrationError::Unknown)?;

    tx.commit().await.map_err(|_| RegistrationError::Unknown)?;
    Ok((SetCookieSession(session), Redirect::to("/")))
}
