//! This module implements the core concepts of our web module.

use axum::extract::FromRef;
use axum_extra::extract::cookie;

use crate::domain::database::AnyDatabase;

mod auth;
mod locale;
pub mod middleware;
mod statics;
mod redirect;
mod htmx;

pub use auth::*;
pub use locale::*;
pub use redirect::*;
pub use htmx::Hx;
pub use statics::{url as static_file_url, Statics};

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
