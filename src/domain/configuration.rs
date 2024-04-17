//! This module contains the interface for configuring the application.

use config::{Config, Environment};

/// The prefix used for every single environment variable.
const ENV_PREFIX: &str = "PLUTO";

/// Separator character used for every single environment variable.
const ENV_SEPARATOR: &str = "__";

/// A representation of a configuration.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Configuration {}

impl Configuration {
    /// Loads in a configuration from environment variables.
    ///
    /// NOTE: This function will automagically load in `.env` file if it exists.
    #[tracing::instrument]
    pub fn try_load() -> Option<Self> {
        // phase 1: load .env if exists
        tracing::debug!("loading .env file..");
        let _ = dotenv::dotenv();

        // phase 2: configure a builder and try deserializing the configuration
        tracing::debug!("loading configuration..");
        Config::builder()
            .add_source(
                Environment::default()
                    .ignore_empty(true)
                    .prefix(ENV_PREFIX)
                    .prefix_separator(ENV_SEPARATOR)
                    .convert_case(config::Case::UpperSnake)
                    .separator(ENV_SEPARATOR),
            )
            .build()
            .ok()?
            .try_deserialize()
            .ok()
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {}
    }
}
