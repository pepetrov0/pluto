//! Implements transaction component

use axum::async_trait;
use chrono::NaiveDateTime;
use sqlx::{prelude::FromRow, PgPool};

/// Represents a transaction
#[derive(Debug, Clone, FromRow)]
pub struct Transaction {
    pub id: String,
    pub note: String,
    pub credit_account: String,
    pub debit_account: String,
    pub credit_asset: String,
    pub debit_asset: String,
    pub credit_stamp: NaiveDateTime,
    pub debit_stamp: NaiveDateTime,
    pub credit_amount: i64,
    pub debit_amount: i64,
    pub credit_settled: bool,
    pub debit_settled: bool,
}

// Represents a transaction repository
#[async_trait]
pub trait TransactionRepository: Send + Sync {
}

impl TransactionRepository for PgPool {
}