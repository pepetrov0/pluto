//! Implements errors related to account creation

use axum::response::{IntoResponse, Redirect};

/// Represents an account creation error
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AccountCreationError {
    InvalidName,
    InvalidCsrf,
    Unknown,
}

impl IntoResponse for AccountCreationError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AccountCreationError::InvalidName => {
                Redirect::to("/new-account?error=invalid-name").into_response()
            }
            AccountCreationError::InvalidCsrf => {
                Redirect::to("/new-account?error=invalid-csrf").into_response()
            }
            AccountCreationError::Unknown => {
                Redirect::to("/new-account?error=unknown").into_response()
            }
        }
    }
}
