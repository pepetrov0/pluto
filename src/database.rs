use std::time::Duration;

use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::{config::Configuration, RouterState};

pub type Database = PgPool;

pub async fn connect(cfg: &Configuration) -> Result<Database, sqlx::Error> {
    // connect to database
    let pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(5))
        .max_connections(8)
        .max_lifetime(Option::Some(Duration::from_secs(60)))
        .connect(cfg.database_url.as_str())
        .await?;

    // run migrations
    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}

#[async_trait]
impl FromRequestParts<RouterState> for Database {
    type Rejection = ();

    async fn from_request_parts(
        _: &mut Parts,
        state: &RouterState,
    ) -> Result<Self, Self::Rejection> {
        Ok(state.database.clone())
    }
}
