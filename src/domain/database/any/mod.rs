//! This module exists simply to wrap all of the other database layer implementations
//! so that the service layer can work with a single implementation, instead of relying 
//! on dynamic dispatch.

use axum::async_trait;

use crate::domain::configuration::Configuration;

use super::{
    postgres::{PgDatabase, PgTransaction},
    sqlite::{SqliteDatabase, SqliteTransaction},
    Database, Transaction,
};

mod sessions;
mod users;

/// A generic database interface for connecting to various databases.
#[derive(Clone)]
pub enum AnyDatabase {
    /// A SQLite database.
    Sqlite(SqliteDatabase),
    /// A PostgreSQL database.
    Pg(PgDatabase),
}

/// A generic transaction interface.
pub enum AnyTransaction {
    /// A SQLite database.
    Sqlite(SqliteTransaction),
    /// A PostgreSQL database.
    Pg(PgTransaction),
}

impl AnyDatabase {
    /// Creates a in-memory SQLite database
    pub async fn in_memory() -> super::Result<Self> {
        SqliteDatabase::connect("sqlite::memory:")
            .await
            .map(Self::Sqlite)
    }

    /// Connects to the database provided by the configuration
    #[tracing::instrument]
    pub async fn connect_with_configuration(config: &Configuration) -> super::Result<Self> {
        match config.database {
            Some(ref url) => Self::connect(url.as_str()).await,
            None => {
                tracing::warn!(
                    "no database was configured! using a in-memory SQLite database instead.."
                );
                Self::connect("sqlite::memory:").await
            }
        }
    }
}

#[async_trait]
impl Database for AnyDatabase {
    type Tx = AnyTransaction;

    #[tracing::instrument]
    async fn connect(url: &str) -> super::Result<Self> {
        if url.starts_with("postgresql:") {
            return PgDatabase::connect(url).await.map(Self::Pg);
        }

        if url.starts_with("sqlite:") {
            return SqliteDatabase::connect(url).await.map(Self::Sqlite);
        }

        Err(super::Error::Message("unknown database".to_string()))
    }

    #[tracing::instrument(skip(self))]
    async fn begin(&self) -> super::Result<Self::Tx> {
        match self {
            AnyDatabase::Sqlite(v) => v.begin().await.map(AnyTransaction::Sqlite),
            AnyDatabase::Pg(v) => v.begin().await.map(AnyTransaction::Pg),
        }
    }

    #[tracing::instrument(skip(self))]
    async fn close(self) {
        match self {
            AnyDatabase::Sqlite(v) => v.close().await,
            AnyDatabase::Pg(v) => v.close().await,
        }
    }
}

#[async_trait]
impl Transaction for AnyTransaction {
    #[tracing::instrument(skip(self))]
    async fn commit(self) -> super::Result<()> {
        match self {
            AnyTransaction::Sqlite(v) => v.commit().await,
            AnyTransaction::Pg(v) => v.commit().await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn rollback(self) {
        match self {
            AnyTransaction::Sqlite(v) => v.rollback().await,
            AnyTransaction::Pg(v) => v.rollback().await,
        }
    }
}
