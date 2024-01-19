//! Implements configuration

use config::Config;

/// Basic configuration structure providing needed variables
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Configuration {
    /// A secret for generating cookie jar keys
    #[serde(skip_serializing)]
    pub cookie_jar_secret: Option<String>,
    /// Session cookie name
    pub session_cookie_name: Option<String>,
    /// Database URL to connect to
    #[serde(skip_serializing)]
    pub database_url: String,
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
