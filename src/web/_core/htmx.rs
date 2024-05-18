//! This module provides utility for working with HTMX requests.

use std::convert::Infallible;

use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

use super::GlobalState;

/// Provides flags related to HTMX requests.
#[derive(Debug, Clone, Copy)]
pub struct Hx {
    pub request: bool,
    pub boosted: bool,
}

#[async_trait]
impl FromRequestParts<GlobalState> for Hx {
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &GlobalState) -> Result<Self, Self::Rejection> {
        let request = parts.headers.contains_key("HX-Request");
        let boosted = parts.headers.contains_key("HX-Boosted");
        Ok(Self { request, boosted })
    }
}
