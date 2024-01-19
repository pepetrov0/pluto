//! Implements error enums relevant to user registration

use axum::response::{IntoResponse, Redirect};

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LoginError {
    InvalidCredentials,
    Unknown,
}

impl IntoResponse for LoginError {
    fn into_response(self) -> axum::response::Response {
        match self {
            LoginError::InvalidCredentials => {
                Redirect::to("/login?error=invalid-credentials").into_response()
            }
            LoginError::Unknown => Redirect::to("/login?error=unknown").into_response(),
        }
    }
}
