//! This module contains the interface to connect and communicate with a
//! database.

use async_trait::async_trait;

mod postgres;

pub use postgres::PgDatabase;

/// A maximum pool connection count.
const MAX_POOL_CONNECTIONS: u32 = 8;

/// A maximum lifetime for an idle connection of the pool.
const MAX_POOL_IDLE_LIFETIME: u64 = 1;

/// A maximum lifetime of a database pool connection in minutes.
/// Used to forcefully close connections after 5 minutes to allow the
/// database engine to cleanup resources associated to the connection.
const MAX_POOL_CONNECTION_LIFETIME: u64 = 5;

/// An alias for a boxed transaction.
pub type BoxedTransaction = Box<dyn Transaction>;

/// A trait implemented for all database types and is the base interface
/// for all databases. Provides a way to connect to a database, to begin
/// a transaction and to close the connection to the database.
#[async_trait]
pub trait Database: Sized {
    /// Connects to a database with the given URL.
    ///
    /// Keep in mind that the URL should provide the relevant database protocol!
    /// (e.g. `postgresql://user:password@hostname:port/database`).
    async fn connect(url: &str) -> Option<Self>;

    /// Begins a transaction and returns it boxed as optional.
    ///
    /// Returns `None` in case of an error.
    async fn begin(&self) -> Option<BoxedTransaction>;

    /// Closes that database connection
    async fn close(self);
}

/// A trait implemented by all database transaction abstractions.
/// Provides basic transaction functionalities such as commiting and rolling
/// a transction back.
#[async_trait]
pub trait Transaction {
    /// Commits the transaction.
    /// If any error occured, returns `false`.
    async fn commit(self) -> bool;

    /// Rolls back the transaction
    async fn rollback(self);
}
