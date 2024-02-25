use axum::{extract::State, response::Redirect, Form};
use serde::Deserialize;

use crate::{
    auth::principal::AuthPrincipal,
    core::database::WriteRepository,
    domain::{self, assets::AssetType, csrf_tokens},
    AppState,
};

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
    let mut repository = WriteRepository::from_pool(&state.database)
        .await
        .ok_or(AssetCreationError::Unknown)?;

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
    if !csrf_tokens::verify(
        &mut repository,
        &details.csrf,
        &user,
        super::CSRF_TOKEN_USAGE,
    )
    .await
    .unwrap_or(false)
    {
        return Err(AssetCreationError::InvalidCsrf);
    }

    domain::assets::create(
        &mut repository,
        &details.ticker,
        details.symbol.as_deref(),
        &details.label,
        details.precision,
        details.atype,
    )
    .await
    .map_err(AssetCreationError::from)?;

    repository
        .commit()
        .await
        .ok_or(AssetCreationError::Unknown)?;
    Ok(Redirect::to("/assets?created=true"))
}
