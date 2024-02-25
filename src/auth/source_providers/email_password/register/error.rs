use axum::response::{IntoResponse, Redirect};

use crate::domain::users::UserCreationError;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RegistrationError {
    InvalidEmail,
    PasswordTooShort,
    PasswordsMismatch,
    EmailTaken,
    InvalidFavoriteAsset,
    Unknown,
}

impl From<UserCreationError> for RegistrationError {
    fn from(err: UserCreationError) -> Self {
        match err {
            UserCreationError::InvalidEmail => RegistrationError::InvalidEmail,
            UserCreationError::EmailTaken => RegistrationError::EmailTaken,
            UserCreationError::Unknown => RegistrationError::Unknown,
        }
    }
}

impl IntoResponse for RegistrationError {
    fn into_response(self) -> axum::response::Response {
        match self {
            RegistrationError::InvalidEmail => {
                Redirect::to("/register?error=invalid-email").into_response()
            }
            RegistrationError::PasswordTooShort => {
                Redirect::to("/register?error=password-too-short").into_response()
            }
            RegistrationError::PasswordsMismatch => {
                Redirect::to("/register?error=passwords-mismatch").into_response()
            }
            RegistrationError::EmailTaken => {
                Redirect::to("/register?error=email-taken").into_response()
            }
            RegistrationError::InvalidFavoriteAsset => {
                Redirect::to("/register?error=invalid-favorite-asset").into_response()
            }
            RegistrationError::Unknown => Redirect::to("/register?error=unknown").into_response(),
        }
    }
}
