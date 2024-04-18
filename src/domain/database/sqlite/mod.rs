use std::{str::FromStr, time::Duration};

use async_trait::async_trait;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Sqlite, SqlitePool, Transaction,
};

/// A SQLite database interface wrapping a pool.
#[derive(Clone)]
pub struct SqliteDatabase(SqlitePool);

/// A SQLite transaction.
pub struct SqliteTransaction(Transaction<'static, Sqlite>);

#[async_trait]
impl super::Database for SqliteDatabase {
    type Tx = SqliteTransaction;

    #[tracing::instrument]
    async fn connect(file: &str) -> Option<Self> {
        tracing::info!("connecting to SQLite..");
        let options = SqliteConnectOptions::from_str(file)
            .map_err(|e| tracing::error!("error while parsing SQLite connection: {e:?}"))
            .ok()?
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
            .map_err(|e| tracing::error!("error while opening SQLite pool: {e:?}"))
            .ok()?;

        tracing::info!("running SQLite migrations..");
        sqlx::migrate!("migrations/sqlite").run(&pool).await.ok()?;

        tracing::info!("SQLite ready!");
        Some(Self(pool))
    }

    async fn begin(&self) -> Option<Self::Tx> {
        Some(SqliteTransaction(self.0.begin().await.ok()?))
    }

    #[tracing::instrument(skip(self))]
    async fn close(self) {
        tracing::info!("closing SQLite database..");
        self.0.close().await;
    }
}

#[async_trait]
impl super::Transaction for SqliteTransaction {
    async fn commit(self) -> bool {
        self.0.commit().await.is_ok()
    }

    async fn rollback(self) {
        let _ = self.0.rollback().await;
    }
}
