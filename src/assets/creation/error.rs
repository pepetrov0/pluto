use axum::response::{IntoResponse, Redirect};

use crate::domain;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AssetCreationError {
    InvalidLabel,
    InvalidTicker,
    InvalidSymbol,
    InvalidPrecision,
    InvalidCsrf,
    AlreadyExists,
    Unknown,
}

impl IntoResponse for AssetCreationError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AssetCreationError::InvalidLabel => {
                Redirect::to("/new-asset?error=invalid-label").into_response()
            }
            AssetCreationError::InvalidTicker => {
                Redirect::to("/new-asset?error=invalid-ticker").into_response()
            }
            AssetCreationError::InvalidSymbol => {
                Redirect::to("/new-asset?error=invalid-symbol").into_response()
            }
            AssetCreationError::InvalidPrecision => {
                Redirect::to("/new-asset?error=invalid-precision").into_response()
            }
            AssetCreationError::InvalidCsrf => {
                Redirect::to("/new-asset?error=invalid-csrf").into_response()
            }
            AssetCreationError::AlreadyExists => {
                Redirect::to("/new-asset?error=already-exists").into_response()
            }
            AssetCreationError::Unknown => Redirect::to("/new-asset?error=unknown").into_response(),
        }
    }
}

impl From<domain::assets::AssetCreationError> for AssetCreationError {
    fn from(value: domain::assets::AssetCreationError) -> Self {
        match value {
            domain::assets::AssetCreationError::Unknown => Self::Unknown,
            domain::assets::AssetCreationError::InvalidLabel => Self::InvalidLabel,
            domain::assets::AssetCreationError::InvalidTicker => Self::InvalidTicker,
            domain::assets::AssetCreationError::InvalidSymbol => Self::InvalidSymbol,
            domain::assets::AssetCreationError::InvalidPrecision => Self::InvalidPrecision,
            domain::assets::AssetCreationError::AlreadyExists => Self::AlreadyExists,
        }
    }
}
