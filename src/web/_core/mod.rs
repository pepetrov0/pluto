//! This module implements the core concepts of our web module.

use axum::extract::FromRef;
use axum_extra::extract::cookie;

use crate::domain::database::AnyDatabase;

mod auth;
mod locale;
pub mod middleware;

pub use auth::*;
pub use locale::*;

/// Shared state.
#[derive(Clone)]
#[allow(unused)]
pub struct State {
    /// The database.
    pub database: AnyDatabase,
    /// The key for the private cookie jars.
    pub key: cookie::Key,
}

impl FromRef<State> for cookie::Key {
    fn from_ref(state: &State) -> Self {
        state.key.clone()
    }
}
