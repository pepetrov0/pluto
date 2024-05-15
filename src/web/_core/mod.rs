use axum::extract::FromRef;
use axum_extra::extract::cookie;

use crate::domain::database::AnyDatabase;

mod auth;
mod locale;
pub mod middleware;

pub use auth::*;
pub use locale::*;

/// State shared between all databases.
#[derive(Clone)]
#[allow(unused)]
pub struct State {
    pub database: AnyDatabase,
    pub key: cookie::Key,
}

impl FromRef<State> for cookie::Key {
    fn from_ref(state: &State) -> Self {
        state.key.clone()
    }
}
