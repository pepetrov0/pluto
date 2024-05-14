use std::{str::FromStr, time::Duration};

use async_trait::async_trait;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Sqlite, SqlitePool, Transaction,
};

mod sessions;
mod users;

/// A SQLite database interface wrapping a pool.
#[derive(Clone)]
pub struct SqliteDatabase(SqlitePool);

/// A SQLite transaction.
pub struct SqliteTransaction(Transaction<'static, Sqlite>);

#[async_trait]
impl super::Database for SqliteDatabase {
    type Tx = SqliteTransaction;

    #[tracing::instrument(err)]
    async fn connect(file: &str) -> super::Result<Self> {
        tracing::info!("connecting to SQLite..");
        let options = SqliteConnectOptions::from_str(file)
            .map_err(super::Error::from)?
            .auto_vacuum(sqlx::sqlite::SqliteAutoVacuum::Incremental)
            .create_if_missing(true)
            .foreign_keys(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .optimize_on_close(true, None);

        let pool = SqlitePoolOptions::new()
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
            .connect_with(options)
            .await
            .map_err(super::Error::from)?;

        tracing::info!("running SQLite migrations..");
        sqlx::migrate!("migrations/sqlite")
            .run(&pool)
            .await
            .map_err(super::Error::from)?;

        tracing::info!("SQLite ready!");
        Ok(Self(pool))
    }

    #[tracing::instrument(err, skip(self))]
    async fn begin(&self) -> super::Result<Self::Tx> {
        tracing::trace!("beginning transaction..");
        self.0
            .begin()
            .await
            .map(SqliteTransaction)
            .map_err(super::Error::from)
    }

    #[tracing::instrument(skip(self))]
    async fn close(self) {
        tracing::info!("closing SQLite database..");
        self.0.close().await;
    }
}

#[async_trait]
impl super::Transaction for SqliteTransaction {
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
