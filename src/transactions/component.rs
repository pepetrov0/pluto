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
    /// Creates a transaction
    #[allow(clippy::too_many_arguments)]
    async fn create_transaction(
        &self,
        note: String,
        credit_account: String,
        debit_account: String,
        credit_asset: String,
        debit_asset: String,
        credit_stamp: NaiveDateTime,
        debit_stamp: NaiveDateTime,
        credit_amount: i64,
        debit_amount: i64,
        credit_settled: bool,
        debit_settled: bool,
    ) -> Option<Transaction>;
}

#[async_trait]
impl TransactionRepository for PgPool {
    async fn create_transaction(
        &self,
        note: String,
        credit_account: String,
        debit_account: String,
        credit_asset: String,
        debit_asset: String,
        credit_stamp: NaiveDateTime,
        debit_stamp: NaiveDateTime,
        credit_amount: i64,
        debit_amount: i64,
        credit_settled: bool,
        debit_settled: bool,
    ) -> Option<Transaction> {
        sqlx::query_as::<_, Transaction>(
            "insert into transactions 
              (id, note, credit_account, debit_account, credit_asset, debit_asset, credit_stamp, debit_stamp, credit_amount, debit_amount, credit_settled, debit_settled) 
              values 
              ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) 
              returning 
              id, note, credit_account, debit_account, credit_asset, debit_asset, credit_stamp, debit_stamp, credit_amount, debit_amount, credit_settled, debit_settled"
            )
            .bind(nanoid::nanoid!())
            .bind(note)
            .bind(credit_account)
            .bind(debit_account)
            .bind(credit_asset)
            .bind(debit_asset)
            .bind(credit_stamp)
            .bind(debit_stamp)
            .bind(credit_amount)
            .bind(debit_amount)
            .bind(credit_settled)
            .bind(debit_settled)
            .fetch_one(self)
            .await
            .ok()
    }
}
