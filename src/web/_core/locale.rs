//! This module implements an extractor to extract the prefered language for the user.

use std::{cmp::Reverse, convert::Infallible};

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header, request::Parts},
};

use super::GlobalState;

/// The default locale that is used if we do not support any of the languages
/// prefered by the user.
const DEFAULT_LOCALE: &str = "en";

/// A structure to represent the prefered locale.
#[derive(Debug, Clone)]
pub struct Locale(String);

impl Locale {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[async_trait]
impl FromRequestParts<GlobalState> for Locale {
    type Rejection = Infallible;

    /// Perform the extraction.
    async fn from_request_parts(
        parts: &mut Parts,
        _state: &GlobalState,
    ) -> Result<Self, Self::Rejection> {
        // gather inputs
        let raw_languages = parts
            .headers
            .get(header::ACCEPT_LANGUAGE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or(DEFAULT_LOCALE);
        let supported_locales = rust_i18n::available_locales!();

        // find intersecting ones and sort descending by quality
        let mut intersecting_locales =
            accept_language::intersection_with_quality(raw_languages, &supported_locales);
        intersecting_locales.sort_by_key(|v| Reverse((v.1 * 100.0) as i32));

        // find prefered locale or default
        let prefered_locale = intersecting_locales
            .first()
            .map(|v| v.0.clone())
            .unwrap_or_else(|| DEFAULT_LOCALE.to_string());
        Ok(Locale(prefered_locale))
    }
}
