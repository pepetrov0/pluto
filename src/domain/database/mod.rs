//! This module contains the interface to connect and communicate with a
//! database.

#![allow(private_bounds)]

use async_trait::async_trait;

// engines
mod any;
mod postgres;
mod sqlite;

// features
pub(in crate::domain) mod users;

pub use any::*;

/// A maximum pool connection count.
const MAX_POOL_CONNECTIONS: u32 = 8;

/// A maximum lifetime for an idle connection of the pool.
const MAX_POOL_IDLE_LIFETIME: u64 = 1;

/// A maximum lifetime of a database pool connection in minutes.
/// Used to forcefully close connections after 5 minutes to allow the
/// database engine to cleanup resources associated to the connection.
const MAX_POOL_CONNECTION_LIFETIME: u64 = 5;

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
    async fn connect(url: &str) -> Option<Self>
    where
        Self: Sized;

    /// Begins a transaction and returns it boxed as optional.
    ///
    /// Returns `None` in case of an error.
    async fn begin(&self) -> Option<Self::Tx>;

    /// Closes that database connection
    async fn close(self);
}

/// A trait implemented by all database transaction abstractions.
/// Provides basic transaction functionalities such as commiting and rolling
/// a transction back.
#[async_trait]
pub trait Transaction: users::Users {
    /// Commits the transaction.
    /// If any error occured, returns `false`.
    async fn commit(self) -> bool;

    /// Rolls back the transaction
    async fn rollback(self);
}
