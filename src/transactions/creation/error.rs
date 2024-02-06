//! Implements error enums relevant to user registration

use axum::response::{IntoResponse, Redirect};

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TransactionCreationError {
    InvalidNote,
    AccountsNotOwned,
    MatchingAccounts,
    MissingCreditAsset,
    MissingDebitAsset,
    InvalidCreditAsset,
    InvalidDebitAsset,
    MissingCreditAmount,
    MissingDebitAmount,
    Unknown,
}

impl IntoResponse for TransactionCreationError {
    fn into_response(self) -> axum::response::Response {
        match self {
            TransactionCreationError::InvalidNote => {
                Redirect::to("/new-transaction?error=invalid-note").into_response()
            }
            TransactionCreationError::AccountsNotOwned => {
                Redirect::to("/new-transaction?error=accounts-not-owned").into_response()
            }
            TransactionCreationError::MatchingAccounts => {
                Redirect::to("/new-transaction?error=matching-accounts").into_response()
            }
            TransactionCreationError::MissingCreditAsset => {
                Redirect::to("/new-transaction?error=missing-credit-asset").into_response()
            }
            TransactionCreationError::MissingDebitAsset => {
                Redirect::to("/new-transaction?error=missing-debit-asset").into_response()
            }
            TransactionCreationError::InvalidCreditAsset => {
                Redirect::to("/new-transaction?error=invalid-credit-asset").into_response()
            }
            TransactionCreationError::InvalidDebitAsset => {
                Redirect::to("/new-transaction?error=invalid-debit-asset").into_response()
            }
            TransactionCreationError::MissingCreditAmount => {
                Redirect::to("/new-transaction?error=missing-credit-amount").into_response()
            }
            TransactionCreationError::MissingDebitAmount => {
                Redirect::to("/new-transaction?error=missing-debit-amount").into_response()
            }
            TransactionCreationError::Unknown => {
                Redirect::to("/new-transaction?error=unknown").into_response()
            }
        }
    }
}
