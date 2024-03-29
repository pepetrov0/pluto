use axum::{extract::State, response::Redirect, Form};
use chrono_tz::Tz;

use crate::{
    auth::{principal::NoAuthPrincipal, session_providers::cookie::SetCookieSession},
    core::database::WriteRepository,
    domain::{self, accounts::AccountCreationError, sessions::SessionCreationError},
    AppState,
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
            .map_err(|_| RegistrationError::Unknown)?
            .ok_or(RegistrationError::InvalidFavoriteAsset)?;

    // parse timezone
    let timezone = Tz::from_str_insensitive(&details.timezone).unwrap_or_default();

    // create default account
    let favorite_account = domain::accounts::create(&mut repository, DEFAULT_ACCOUNT_NAME)
        .await
        .map_err(RegistrationError::from)?;

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
    domain::accounts_ownerships::create(&mut repository, &user, &favorite_account)
        .await
        .map_err(RegistrationError::from)?;

    // create a session
    let session = domain::sessions::create(&mut repository, &user)
        .await
        .map_err(RegistrationError::from)?;

    repository
        .commit()
        .await
        .ok_or(RegistrationError::Unknown)?;
    Ok((SetCookieSession(session), Redirect::to("/")))
}

impl From<SessionCreationError> for RegistrationError {
    fn from(value: SessionCreationError) -> Self {
        match value {
            SessionCreationError::Unknown => Self::Unknown,
        }
    }
}

impl From<AccountCreationError> for RegistrationError {
    fn from(value: AccountCreationError) -> Self {
        match value {
            AccountCreationError::Unknown => Self::Unknown,
            AccountCreationError::InvalidName => Self::Unknown,
        }
    }
}
