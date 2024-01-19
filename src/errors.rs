//! Implements application error handling

use axum::{response::{IntoResponse, Redirect}, http::StatusCode};

pub enum AppError {
    NotFound,
    Unauthorized,
    NotAllowedHere,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND.into_response(),
            AppError::Unauthorized => Redirect::to("/login").into_response(),
            AppError::NotAllowedHere => Redirect::to("/").into_response(),
        }
    }
}