//! Implements asset creation API

use axum::{extract::State, response::Redirect, Form};
use chrono::Utc;
use serde::Deserialize;

use crate::{assets::component::AssetType, auth::principal::AuthPrincipal, csrf_tokens, AppState};

use super::error::AssetCreationError;

#[derive(Debug, Clone, Deserialize)]
pub struct NewAssetForm {
    pub label: String,
    pub ticker: String,
    pub symbol: Option<String>,
    pub precision: i16,
    pub atype: AssetType,
    pub csrf: String,
}

pub async fn handler(
    AuthPrincipal(user): AuthPrincipal,
    State(state): State<AppState>,
    Form(details): Form<NewAssetForm>,
) -> Result<Redirect, AssetCreationError> {
    let details = NewAssetForm {
        label: details.label.trim().to_owned(),
        ticker: details.ticker.trim().to_lowercase().to_owned(),
        symbol: details
            .symbol
            .map(|v| v.trim().to_owned())
            .filter(|v| !v.is_empty()),
        precision: details.precision,
        atype: details.atype,
        csrf: details.csrf.trim().to_owned(),
    };

    // check csrf
    let csrf = state
        .csrf_token_repository
        .consume_csrf_token(details.csrf.as_str())
        .await;
    if csrf
        .filter(|v| v.user == user.id)
        .filter(|v| v.usage == super::CSRF_TOKEN_USAGE)
        .filter(|v| {
            (Utc::now().naive_utc() - v.created_at).num_seconds() < csrf_tokens::CSRF_TOKEN_LIFETIME
        })
        .is_none()
    {
        return Err(AssetCreationError::InvalidCsrf);
    }

    // validate label
    if details.label.is_empty() || details.label.len() > 200 {
        return Err(AssetCreationError::InvalidLabel);
    }

    // validate label
    if details.ticker.is_empty() || details.ticker.len() > 8 {
        return Err(AssetCreationError::InvalidTicker);
    }

    // validate symbol
    if let Some(symbol) = &details.symbol {
        if symbol.len() > 8 {
            return Err(AssetCreationError::InvalidSymbol);
        }
    }

    // validate precision
    if details.precision < 0 || details.precision > 4 {
        return Err(AssetCreationError::InvalidPrecision);
    }

    if state
        .asset_repository
        .find_asset_by_ticker(&details.ticker)
        .await
        .is_some()
    {
        return Err(AssetCreationError::AlreadyExists);
    }

    state
        .asset_repository
        .create_asset(
            details.ticker,
            details.symbol,
            details.label,
            details.precision,
            details.atype,
        )
        .await
        .ok_or(AssetCreationError::Unknown)?;

    Ok(Redirect::to("/assets?created=true"))
}
