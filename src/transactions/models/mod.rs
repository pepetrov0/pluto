use chrono_tz::Tz;
use either::Either;

use crate::{
    accounts::{component::Account, ownership::AccountOwnership},
    assets::component::Asset,
    users::User,
    DATE_TIME_FORMAT, DATE_TIME_FORMAT_NICE,
};

use super::component::Transaction;

#[derive(Debug)]
pub struct ResolvedTransaction {
    pub note: String,
    pub credit: Either<Account, Vec<User>>,
    pub debit: Either<Account, Vec<User>>,
    pub credit_asset: Asset,
    pub debit_asset: Asset,
    pub credit_amount: f64,
    pub debit_amount: f64,
    pub credit_stamp: String,
    pub debit_stamp: String,
    pub credit_stamp_nice: String,
    pub debit_stamp_nice: String,
}

impl Transaction {
    pub fn into_resolved(
        self,
        tz: &Tz,
        user: &User,
        users: &[User],
        assets: &[Asset],
        accounts: &[Account],
        ownerships: &[AccountOwnership],
    ) -> Option<ResolvedTransaction> {
        // ownership
        let credit_owned = ownerships
            .iter()
            .filter(|&o| o.account == self.credit_account)
            .count()
            > 0;
        let debit_owned = ownerships
            .iter()
            .filter(|&o| o.account == self.debit_account)
            .count()
            > 0;
        let credit_owned_by_self = ownerships
            .iter()
            .filter(|&o| o.usr == user.id)
            .filter(|&o| o.account == self.credit_account)
            .count()
            > 0;
        let debit_owned_by_self = ownerships
            .iter()
            .filter(|&o| o.usr == user.id)
            .filter(|&o| o.account == self.debit_account)
            .count()
            > 0;

        // accounts/users
        let credit = match !credit_owned || credit_owned_by_self {
            true => accounts
                .iter()
                .find(|&a| a.id == self.credit_account)
                .cloned()
                .map(Either::Left)?,
            false => {
                let users = ownerships
                    .iter()
                    .filter(|&o| o.account == self.credit_account)
                    .flat_map(|o| users.iter().find(|&u| u.id == o.usr))
                    .cloned()
                    .collect::<Vec<_>>();
                Either::Right(users)
            }
        };
        let debit = match !debit_owned || debit_owned_by_self {
            true => accounts
                .iter()
                .find(|&a| a.id == self.debit_account)
                .cloned()
                .map(Either::Left)?,
            false => {
                let users = ownerships
                    .iter()
                    .filter(|&o| o.account == self.debit_account)
                    .flat_map(|o| users.iter().find(|&u| u.id == o.usr))
                    .cloned()
                    .collect::<Vec<_>>();
                Either::Right(users)
            }
        };

        // assets
        let credit_asset = assets
            .iter()
            .find(|&a| a.id == self.credit_asset)
            .cloned()?;
        let debit_asset = assets.iter().find(|&a| a.id == self.debit_asset).cloned()?;

        // amounts
        let credit_amount = self.credit_amount as f64 / 10f64.powi(credit_asset.precision.into());
        let debit_amount = self.debit_amount as f64 / 10f64.powi(debit_asset.precision.into());

        // stamps
        let credit_stamp = self.credit_stamp.and_utc().with_timezone(tz);
        let debit_stamp = self.debit_stamp.and_utc().with_timezone(tz);
        let credit_stamp_nice = credit_stamp.format(DATE_TIME_FORMAT_NICE).to_string();
        let debit_stamp_nice = debit_stamp.format(DATE_TIME_FORMAT_NICE).to_string();
        let credit_stamp = credit_stamp.format(DATE_TIME_FORMAT).to_string();
        let debit_stamp = debit_stamp.format(DATE_TIME_FORMAT).to_string();

        let t = ResolvedTransaction {
            note: self.note,
            credit,
            debit,
            credit_asset,
            debit_asset,
            credit_amount,
            debit_amount,
            credit_stamp,
            debit_stamp,
            credit_stamp_nice,
            debit_stamp_nice,
        };

        Some(t)
    }
}
