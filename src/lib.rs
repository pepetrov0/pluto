use std::sync::Arc;

use accounts::{component::AccountRepository, ownership::AccountOwnershipRepository};
use assets::component::AssetRepository;
use auth::{password_hasher::PasswordHasher, session::SessionRepository};
use axum::{extract::FromRef, middleware, Router};
use axum_extra::extract::cookie::Key;
use csrf_tokens::CsrfTokenRepository;
use database::Pool;
use tower_http::trace::TraceLayer;
use transactions::{component::TransactionRepository, entries::EntryRepository};
use user::UserRepository;

pub mod accounts;
pub mod assets;
pub mod auth;
pub mod compression;
pub mod config;
pub mod content_security_policy;
pub mod csrf_tokens;
pub mod dashboard;
pub mod database;
pub mod healthcheck;
pub mod imkvs;
pub mod shutdown;
pub mod static_files;
pub mod templates;
pub mod transactions;
pub mod user;
pub mod validation;

#[derive(Clone)]
pub struct AppState {
    pub configuration: config::Configuration,
    pub cookie_jar_key: Key,
    pub database: Arc<dyn Pool>,
    pub password_hasher: Arc<dyn PasswordHasher>,
    pub csrf_token_repository: Arc<dyn CsrfTokenRepository>,
    pub user_repository: Arc<dyn UserRepository>,
    pub session_repository: Arc<dyn SessionRepository>,
    pub asset_repository: Arc<dyn AssetRepository>,
    pub account_repository: Arc<dyn AccountRepository>,
    pub account_ownership_repository: Arc<dyn AccountOwnershipRepository>,
    pub transaction_repository: Arc<dyn TransactionRepository>,
    pub entry_repository: Arc<dyn EntryRepository>,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.cookie_jar_key.clone()
    }
}

pub fn router(state: AppState) -> Router {
    Router::new()
        // healthcheck
        .merge(healthcheck::router())
        // auth router
        .merge(auth::router())
        // dashboard
        .merge(dashboard::router())
        // assets
        .merge(assets::router())
        // acounts
        .merge(accounts::router())
        // transactions
        .merge(transactions::router())
        // static files
        .merge(static_files::router())
        // auth middlewares
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth::session_providers::cookie::middleware,
        ))
        .layer(compression::default())
        .layer(middleware::from_fn(content_security_policy::middleware))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
