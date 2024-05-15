//! This module contains the interface for configuring the application.

use config::{Config, Environment};

/// The prefix used for every single environment variable.
const ENV_PREFIX: &str = "PLUTO";

/// Separator character used for every single environment variable.
const ENV_SEPARATOR: &str = "__";

/// A representation of a configuration.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Configuration {
    /// A secret used for key derivation.
    pub secret: Option<String>,
    /// Options for configuring a database.
    pub database: Option<String>,
}

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
        let config = Config::builder()
            .add_source(
                Environment::default()
                    .ignore_empty(true)
                    .prefix(ENV_PREFIX)
                    .prefix_separator(ENV_SEPARATOR)
                    .convert_case(config::Case::UpperSnake)
                    .separator(ENV_SEPARATOR),
            )
            .build()
            .map_err(|e| tracing::error!("unable to read configuration: {e:?}"))
            .ok()?
            .try_deserialize()
            .map_err(|e| tracing::error!("error deserializing configuration: {e:?}"))
            .ok();

        match config {
            Some(ref cfg) => tracing::info!("loaded in configuration: {cfg:?}"),
            None => tracing::error!("failed loading in configuration!"),
        };

        config
    }
}
