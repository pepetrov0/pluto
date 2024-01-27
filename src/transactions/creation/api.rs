//! Implements transaction creation API

use axum::{response::Redirect, Form};
use serde::Deserialize;

use crate::auth::principal::AuthPrincipal;

#[derive(Debug, Deserialize)]
pub struct NewTransactionForm {
    pub note: String,
    pub credit_account: String,
    #[serde(default)]
    pub create_credit_account: bool,
    pub debit_account: String,
    #[serde(default)]
    pub create_debit_account: bool,
    pub asset: Option<String>,
    pub credit_asset: Option<String>,
    pub debit_asset: Option<String>,
    pub amount: Option<f64>,
    pub credit_amount: Option<f64>,
    pub debit_amount: Option<f64>,
    pub date: String,
}

pub async fn handler(_: AuthPrincipal, Form(details): Form<NewTransactionForm>) -> Redirect {
    println!("{:#?}", details);
    Redirect::to("/transactions")
}
