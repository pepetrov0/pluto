use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::config::Configuration;

pub trait Pool: Send + Sync {
    fn is_open(&self) -> bool;
    fn size(&self) -> u32;
    fn num_idle(&self) -> u32;
}

pub async fn connect(cfg: &Configuration) -> Result<PgPool, sqlx::Error> {
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

impl Pool for PgPool {
    fn is_open(&self) -> bool {
        !self.is_closed()
    }

    fn size(&self) -> u32 {
        self.size()
    }

    fn num_idle(&self) -> u32 {
        self.num_idle() as u32
    }
}
