//! This module implements core elements needed for implementation of the web UI.

use crate::domain::database::AnyDatabase;

pub mod cache_control;

/// State shared between all databases.
#[derive(Clone)]
pub struct State {
    pub database: AnyDatabase,
}
