use config::Config;
use rand::{distributions, rngs::OsRng, Rng};

const DEFAULT_SECRET_LENGTH: usize = 64;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Configuration {
    #[serde(skip_serializing)]
    #[serde(default = "random_secret")]
    pub secret: String,

    pub session_cookie_name: Option<String>,

    #[serde(skip_serializing)]
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

fn random_secret() -> String {
    let bytes = (&mut OsRng)
        .sample_iter(distributions::Alphanumeric)
        .take(DEFAULT_SECRET_LENGTH)
        .collect();
    String::from_utf8(bytes).expect("msg")
}
