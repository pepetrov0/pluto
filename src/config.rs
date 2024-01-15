use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use config::Config;

use crate::RouterState;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Configuration {
    pub database_url: String,
}

impl Configuration {
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

#[async_trait]
impl FromRequestParts<RouterState> for Configuration {
    type Rejection = ();

    async fn from_request_parts(
        _: &mut Parts,
        state: &RouterState,
    ) -> Result<Self, Self::Rejection> {
        Ok(state.configuration.clone())
    }
}
