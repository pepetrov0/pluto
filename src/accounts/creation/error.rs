use axum::response::{IntoResponse, Redirect};

use crate::domain::accounts_ownerships::AccountOwnershipCreationError;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AccountCreationError {
    InvalidName,
    InvalidCsrf,
    Unknown,
}

impl From<AccountOwnershipCreationError> for AccountCreationError {
    fn from(value: AccountOwnershipCreationError) -> Self {
        match value {
            AccountOwnershipCreationError::Unknown => Self::Unknown,
        }
    }
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
