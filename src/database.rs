//! Implements a database connection

use std::{fmt::Debug, time::Duration};

use axum::async_trait;
use sqlx::{pool::PoolConnection, postgres::PgPoolOptions, Acquire, PgPool, Pool, Transaction};

use crate::config::Configuration;

#[async_trait]
pub trait DatabaseRepository: Debug + Send {
    async fn acquire(&mut self) -> Option<&mut <sqlx::Postgres as sqlx::Database>::Connection>;
}

pub trait ReadonlyDatabaseRepository: DatabaseRepository {}

pub trait WriteDatabaseRepository: DatabaseRepository + ReadonlyDatabaseRepository {}

#[derive(Debug)]
pub struct ReadonlyRepository(PoolConnection<sqlx::Postgres>);

impl ReadonlyRepository {
    pub async fn from_pool(pool: &Pool<sqlx::Postgres>) -> Option<Self> {
        pool.acquire().await.ok().map(Self)
    }
}

impl ReadonlyDatabaseRepository for ReadonlyRepository {}
#[async_trait]
impl DatabaseRepository for ReadonlyRepository {
    async fn acquire(&mut self) -> Option<&mut <sqlx::Postgres as sqlx::Database>::Connection> {
        self.0.acquire().await.ok()
    }
}

#[derive(Debug)]
pub struct WriteRepository(Transaction<'static, sqlx::Postgres>);

impl WriteRepository {
    pub async fn from_pool(pool: &Pool<sqlx::Postgres>) -> Option<Self> {
        pool.begin().await.ok().map(Self)
    }

    pub async fn commit(self) -> Option<()> {
        self.0.commit().await.ok()
    }
}

impl ReadonlyDatabaseRepository for WriteRepository {}
impl WriteDatabaseRepository for WriteRepository {}
#[async_trait]
impl DatabaseRepository for WriteRepository {
    async fn acquire(&mut self) -> Option<&mut <sqlx::Postgres as sqlx::Database>::Connection> {
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
