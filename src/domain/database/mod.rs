//! This module contains the interface to connect and communicate with a
//! database.

#![allow(private_bounds)]

use async_trait::async_trait;

// engines
mod any;
mod postgres;
mod sqlite;

pub use any::*;

use super::{sessions::SessionsRepository, users::UsersRepository};

/// A maximum pool connection count.
const MAX_POOL_CONNECTIONS: u32 = 8;

/// A maximum lifetime for an idle connection of the pool.
const MAX_POOL_IDLE_LIFETIME: u64 = 1;

/// A maximum lifetime of a database pool connection in minutes.
/// Used to forcefully close connections after 5 minutes to allow the
/// database engine to cleanup resources associated to the connection.
const MAX_POOL_CONNECTION_LIFETIME: u64 = 5;

/// An error that might occur when working with the database layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// A general error described by a message.
    Message(String),
    /// An error indicating that no row was found by the query.
    RowNotFound,
}

/// An alias to a result of a type and database error.
pub type Result<T> = std::result::Result<T, Error>;

/// A trait implemented for all database types and is the base interface
/// for all databases. Provides a way to connect to a database, to begin
/// a transaction and to close the connection to the database.
#[async_trait]
pub trait Database: Clone {
    /// The transaction type associated for the database.
    type Tx: Transaction;

    /// Connects to a database with the given URL.
    ///
    /// Keep in mind that the URL should provide the relevant database protocol!
    /// (e.g. `postgresql://user:password@hostname:port/database`).
    async fn connect(url: &str) -> Result<Self>
    where
        Self: Sized;

    /// Begins a transaction and returns it boxed as optional.
    ///
    /// Returns `None` in case of an error.
    async fn begin(&self) -> Result<Self::Tx>;

    /// Closes that database connection
    async fn close(self);
}

/// A trait implemented by all database transaction abstractions.
/// Provides basic transaction functionalities such as commiting and rolling
/// a transction back.
#[async_trait]
pub trait Transaction: UsersRepository + SessionsRepository {
    /// Commits the transaction.
    /// If any error occured, returns `false`.
    async fn commit(self) -> Result<()>;

    /// Rolls back the transaction
    async fn rollback(self);
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Self::RowNotFound,
            e => Self::Message(e.to_string()),
        }
    }
}

impl From<sqlx::migrate::MigrateError> for Error {
    fn from(value: sqlx::migrate::MigrateError) -> Self {
        Self::Message(value.to_string())
    }
}
