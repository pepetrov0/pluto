use axum::async_trait;
use chrono::NaiveDateTime;

use crate::core::database::{ReadonlyDatabaseRepository, RepositoryError, RepositoryResult, WriteDatabaseRepository};

use super::Transaction;

#[async_trait]
pub trait TransactionReadonlyRepository {
    async fn count_settled_transactions(&mut self, accounts: &[String]) -> RepositoryResult<i64>;

    async fn list_settled_transactions(
        &mut self,
        page_offset: i64,
        page_size: i64,
        accounts: &[String],
    ) -> RepositoryResult<Vec<Transaction>>;

    async fn list_unsettled_transactions(
        &mut self,
        accounts: &[String],
    ) -> RepositoryResult<Vec<Transaction>>;
}

#[async_trait]
pub trait TransactionWriteRepository {
    #[allow(clippy::too_many_arguments)]
    async fn create_transaction(
        &mut self,
        note: &str,
        credit_account: &str,
        debit_account: &str,
        credit_asset: &str,
        debit_asset: &str,
        credit_stamp: NaiveDateTime,
        debit_stamp: NaiveDateTime,
        credit_amount: i64,
        debit_amount: i64,
        credit_settled: bool,
        debit_settled: bool,
    ) -> RepositoryResult<Transaction>;
}

#[async_trait]
impl<T> TransactionReadonlyRepository for T
where
    T: ReadonlyDatabaseRepository + Send + std::fmt::Debug,
{
    #[tracing::instrument(err)]
    async fn count_settled_transactions(&mut self, accounts: &[String]) -> RepositoryResult<i64> {
        if accounts.is_empty() {
            return Ok(0);
        }

        sqlx::query_as::<_, (i64,)>(include_str!("sql/count-settled.pg.sql"))
            .bind(accounts)
            .fetch_one(self.acquire().await?)
            .await
            .map(|(v,)| v)
            .map_err(RepositoryError)
    }

    #[tracing::instrument(err)]
    async fn list_settled_transactions(
        &mut self,
        page_offset: i64,
        page_size: i64,
        accounts: &[String],
    ) -> RepositoryResult<Vec<Transaction>> {
        if accounts.is_empty() {
            return Ok(vec![]);
        }

        sqlx::query_as::<_, Transaction>(include_str!("sql/list-settled.pg.sql"))
            .bind(page_offset)
            .bind(page_size)
            .bind(accounts)
            .fetch_all(self.acquire().await?)
            .await
            .map_err(RepositoryError)
    }

    #[tracing::instrument(err)]
    async fn list_unsettled_transactions(
        &mut self,
        accounts: &[String],
    ) -> RepositoryResult<Vec<Transaction>> {
        if accounts.is_empty() {
            return Ok(vec![]);
        }

        sqlx::query_as::<_, Transaction>(include_str!("sql/list-unsettled.pg.sql"))
            .bind(accounts)
            .fetch_all(self.acquire().await?)
            .await
            .map_err(RepositoryError)
    }
}

#[async_trait]
impl<T> TransactionWriteRepository for T
where
    T: WriteDatabaseRepository + Send + std::fmt::Debug,
{
    #[tracing::instrument(err)]
    async fn create_transaction(
        &mut self,
        note: &str,
        credit_account: &str,
        debit_account: &str,
        credit_asset: &str,
        debit_asset: &str,
        credit_stamp: NaiveDateTime,
        debit_stamp: NaiveDateTime,
        credit_amount: i64,
        debit_amount: i64,
        credit_settled: bool,
        debit_settled: bool,
    ) -> RepositoryResult<Transaction> {
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
            .fetch_one(self.acquire().await?)
            .await
            .map_err(RepositoryError)
    }
}
