//! This module provides the necessary implementation for the application
//! to use PostgreSQL as it data source.

use std::time::Duration;

use async_trait::async_trait;
use sqlx::{postgres::PgPoolOptions, PgPool, Postgres, Transaction};

mod sessions;
mod users;

/// A PostgreSQL database interface wrapping a pool.
#[derive(Clone)]
pub struct PgDatabase(PgPool);

/// A PostgreSQL transaction.
pub struct PgTransaction(Transaction<'static, Postgres>);

#[async_trait]
impl super::Database for PgDatabase {
    type Tx = PgTransaction;

    #[tracing::instrument(err)]
    async fn connect(url: &str) -> super::Result<Self> {
        tracing::info!("connecting to PostgreSQL..");
        let pool = PgPoolOptions::new()
            .max_connections(super::MAX_POOL_CONNECTIONS)
            // Sets the maximum idle connection lifetime.
            // As the constant in super is defined in terms of minutes, we have
            // to multiple here by 60 to get seconds.
            .idle_timeout(Duration::from_secs(super::MAX_POOL_IDLE_LIFETIME * 60))
            // Sets the maximum connection lifetime.
            // As the constant in super is defined in terms of minutes, we have
            // to multiple here by 60 to get seconds.
            .max_lifetime(Duration::from_secs(
                super::MAX_POOL_CONNECTION_LIFETIME * 60,
            ))
            .connect(url)
            .await
            .map_err(super::Error::from)?;

        tracing::info!("running PostgreSQL migrations..");
        sqlx::migrate!("migrations/pg")
            .run(&pool)
            .await
            .map_err(super::Error::from)?;

        tracing::info!("PostgreSQL ready!");
        Ok(Self(pool))
    }

    #[tracing::instrument(err, skip(self))]
    async fn begin(&self) -> super::Result<Self::Tx> {
        tracing::trace!("beginning transaction..");
        self.0
            .begin()
            .await
            .map(PgTransaction)
            .map_err(super::Error::from)
    }

    #[tracing::instrument(skip(self))]
    async fn close(self) {
        tracing::info!("closing PostgreSQL database..");
        self.0.close().await;
    }
}

#[async_trait]
impl super::Transaction for PgTransaction {
    #[tracing::instrument(err, skip(self))]
    async fn commit(self) -> super::Result<()> {
        tracing::trace!("committing transaction..");
        self.0.commit().await.map_err(super::Error::from)
    }

    #[tracing::instrument(skip(self))]
    async fn rollback(self) {
        tracing::trace!("rolling transaction back..");
        let _ = self.0.rollback().await;
    }
}
