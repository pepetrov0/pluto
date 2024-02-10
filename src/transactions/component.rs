//! Implements transaction component

use axum::async_trait;
use chrono::NaiveDateTime;
use sqlx::{prelude::FromRow, Postgres};

use crate::database::AsExecutor;

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
pub trait TransactionRepository {
    /// Creates a transaction
    #[allow(clippy::too_many_arguments)]
    async fn create_transaction(
        &mut self,
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

    /// Count of transactions
    async fn count_settled_transactions(&mut self, accounts: Vec<String>) -> Option<i64>;

    /// List transactions by page
    async fn list_settled_transactions(
        &mut self,
        page_offset: i64,
        page_size: i64,
        accounts: Vec<String>,
    ) -> Option<Vec<Transaction>>;
}

#[async_trait]
impl<T> TransactionRepository for T
where
    T: AsExecutor<Postgres> + Send + std::fmt::Debug,
{
    // #[tracing::instrument(skip(self))]
    async fn create_transaction(
        &mut self,
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
        sqlx::query_as::<_, Transaction>(include_str!("sql/create.pg.sql"))
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
            .fetch_one(self.as_executor())
            .await
            .map_err(|v| tracing::error!("{:#?}", v))
            .ok()
    }

    #[tracing::instrument]
    async fn count_settled_transactions(&mut self, accounts: Vec<String>) -> Option<i64> {
        if accounts.is_empty() {
            return Some(0);
        }

        sqlx::query_as::<_, (i64,)>(include_str!("sql/count-settled.pg.sql"))
            .bind(&accounts[..])
            .fetch_one(self.as_executor())
            .await
            .map_err(|v| tracing::error!("{:#?}", v))
            .ok()
            .map(|(v,)| v)
    }

    #[tracing::instrument]
    async fn list_settled_transactions(
        &mut self,
        page_offset: i64,
        page_size: i64,
        accounts: Vec<String>,
    ) -> Option<Vec<Transaction>> {
        if accounts.is_empty() {
            return Some(vec![]);
        }

        sqlx::query_as::<_, Transaction>(include_str!("sql/list-settled.pg.sql"))
            .bind(page_offset)
            .bind(page_size)
            .bind(&accounts[..])
            .fetch_all(self.as_executor())
            .await
            .map_err(|v| tracing::error!("{:#?}", v))
            .ok()
    }
}
