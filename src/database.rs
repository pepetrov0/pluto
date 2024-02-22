//! Implements a database connection

use std::{fmt::Debug, time::Duration};

use axum::async_trait;
use sqlx::{pool::PoolConnection, postgres::PgPoolOptions, Acquire, PgPool, Pool, Transaction};

use crate::config::Configuration;

#[async_trait]
pub trait DatabaseRepository<DB: sqlx::Database>: Debug + Send {
    async fn acquire(&mut self) -> Option<&mut <DB as sqlx::Database>::Connection>;
}

pub trait ReadonlyDatabaseRepository<DB: sqlx::Database>: DatabaseRepository<DB> {}

pub trait WriteDatabaseRepository<DB: sqlx::Database>:
    DatabaseRepository<DB> + ReadonlyDatabaseRepository<DB>
{
}

#[derive(Debug)]
pub struct ReadonlyRepository<DB: sqlx::Database>(PoolConnection<DB>);

impl<DB: sqlx::Database> ReadonlyRepository<DB> {
    pub async fn from_pool(pool: &Pool<DB>) -> Option<Self> {
        pool.acquire().await.ok().map(Self)
    }
}

impl<DB: sqlx::Database> ReadonlyDatabaseRepository<DB> for ReadonlyRepository<DB> {}
#[async_trait]
impl<DB: sqlx::Database> DatabaseRepository<DB> for ReadonlyRepository<DB> {
    async fn acquire(&mut self) -> Option<&mut <DB as sqlx::Database>::Connection> {
        self.0.acquire().await.ok()
    }
}

#[derive(Debug)]
pub struct WriteRepository<DB: sqlx::Database>(Transaction<'static, DB>);

impl<DB: sqlx::Database> WriteRepository<DB> {
    pub async fn from_pool(pool: &Pool<DB>) -> Option<Self> {
        pool.begin().await.ok().map(Self)
    }

    pub async fn commit(self) -> Option<()> {
        self.0.commit().await.ok()
    }
}

impl<DB: sqlx::Database> ReadonlyDatabaseRepository<DB> for WriteRepository<DB> {}
impl<DB: sqlx::Database> WriteDatabaseRepository<DB> for WriteRepository<DB> {}
#[async_trait]
impl<DB: sqlx::Database> DatabaseRepository<DB> for WriteRepository<DB> {
    async fn acquire(&mut self) -> Option<&mut <DB as sqlx::Database>::Connection> {
        self.0.acquire().await.ok()
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
