use sqlx::PgPool;

use crate::config::Configuration;

pub type Database = PgPool;

pub async fn connect(cfg: &Configuration) -> Result<Database, sqlx::Error> {
    // connect to database
    let pool = Database::connect(cfg.database_url.as_str()).await?;

    // run migrations
    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}
