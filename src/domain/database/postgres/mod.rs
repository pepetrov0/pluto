use std::time::Duration;

use async_trait::async_trait;
use sqlx::{postgres::PgPoolOptions, PgPool, Postgres, Transaction};

/// A PostgreSQL database interface wrapping a pool.
#[derive(Clone)]
pub struct PgDatabase(PgPool);

/// A PostgreSQL transaction.
pub struct PgTransaction(Transaction<'static, Postgres>);

#[async_trait]
impl super::Database for PgDatabase {
    type Tx = PgTransaction;

    #[tracing::instrument]
    async fn connect(url: &str) -> Option<Self> {
        tracing::info!("connecting to PostgreSQL..");
        PgPoolOptions::new()
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
            .map_err(|e| tracing::error!("error while opening PostgreSQL pool: {e:?}"))
            .ok()
            .map(Self)
    }

    async fn begin(&self) -> Option<Self::Tx> {
        Some(PgTransaction(self.0.begin().await.ok()?))
    }

    #[tracing::instrument(skip(self))]
    async fn close(self) {
        tracing::info!("closing PostgreSQL database..");
        self.0.close().await;
    }
}

#[async_trait]
impl super::Transaction for PgTransaction {
    async fn commit(self) -> bool {
        self.0.commit().await.is_ok()
    }

    async fn rollback(self) {
        let _ = self.0.rollback().await;
    }
}
