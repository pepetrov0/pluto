//! This module implements the core concepts of our web module.

use axum::extract::FromRef;
use axum_extra::extract::cookie;

use crate::domain::{database::AnyDatabase, Configuration};

mod auth;
mod htmx;
mod locale;
pub mod middleware;
mod statics;

pub use auth::*;
pub use htmx::Hx;
pub use locale::*;
pub use statics::{url as static_file_url, Statics};

/// Shared state.
#[derive(Clone)]
#[allow(unused)]
pub struct GlobalState {
    /// The configuration
    pub cfg: Configuration,
    /// The database.
    pub database: AnyDatabase,
    /// The key for the private cookie jars.
    pub key: cookie::Key,
}

impl FromRef<GlobalState> for cookie::Key {
    fn from_ref(state: &GlobalState) -> Self {
        state.key.clone()
    }
}
