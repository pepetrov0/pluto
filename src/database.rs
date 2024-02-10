//! Implements a database connection

use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, Executor, PgPool, Postgres, Transaction};

use crate::config::Configuration;

pub trait AsExecutor<DB: sqlx::Database> {
    fn as_executor(&mut self) -> impl Executor<'_, Database = DB>;
}

pub trait AsReadonlyExecutor<DB: sqlx::Database>: AsExecutor<DB> {}

pub trait AsWriteExecutor<DB: sqlx::Database>: AsReadonlyExecutor<DB> {}

impl AsReadonlyExecutor<Postgres> for Transaction<'static, Postgres> {}
impl AsWriteExecutor<Postgres> for Transaction<'static, Postgres> {}
impl AsExecutor<Postgres> for Transaction<'static, Postgres> {
    fn as_executor(&mut self) -> impl Executor<'_, Database = Postgres> {
        &mut **self
    }
}

impl AsReadonlyExecutor<Postgres> for PgPool {}
impl AsExecutor<Postgres> for PgPool {
    fn as_executor(&mut self) -> impl Executor<'_, Database = Postgres> {
        &*self
    }
}

/// Connects to a postgres database
pub async fn connect_to_postgres(cfg: &Configuration) -> Result<PgPool, sqlx::Error> {
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
