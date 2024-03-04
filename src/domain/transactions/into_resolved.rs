use chrono_tz::Tz;
use either::Either;

use crate::{
    accounts::ownership::AccountOwnership,
    domain::{accounts::Account, assets::Asset, users::User},
};

use super::ResolvedTransaction;

impl crate::domain::transactions::Transaction {
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
        };

        Some(t)
    }
}
