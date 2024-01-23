//! Implements configuration

use config::Config;
use rand::{distributions, rngs::OsRng, Rng};

const DEFAULT_SECRET_LENGTH: usize = 64;

/// Basic configuration structure providing needed variables
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Configuration {
    /// A secret for generating keys
    #[serde(skip_serializing)]
    #[serde(default = "random_secret")]
    pub secret: String,
    #[serde(default)]
    pub session_driver: SessionDriver,
    /// Session cookie name
    pub session_cookie_name: Option<String>,
    /// Database URL to connect to
    #[serde(skip_serializing)]
    pub database_url: String,
}

/// Represents a session driver
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionDriver {
    InMemory,
    Database,
}

impl Configuration {
    /// Reads the configuration from environment variables (including .env)
    pub fn read() -> Option<Configuration> {
        dotenv::dotenv().ok();

        Config::builder()
            .add_source(
                config::Environment::default()
                    .prefix("PLUTO")
                    .prefix_separator("__")
                    .separator("__"),
            )
            .build()
            .ok()?
            .try_deserialize()
            .ok()
    }
}

fn random_secret() -> String {
    let bytes = (&mut OsRng)
        .sample_iter(distributions::Alphanumeric)
        .take(DEFAULT_SECRET_LENGTH)
        .collect();
    String::from_utf8(bytes).expect("msg")
}

impl Default for SessionDriver {
    fn default() -> Self {
        Self::InMemory
    }
}
