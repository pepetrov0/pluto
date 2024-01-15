use config::Config;

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
