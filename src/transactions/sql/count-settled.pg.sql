select
  count(*)
from
  (
    -- outs
    (
      select
        id,
        note,
        credit_account,
        debit_account,
        credit_asset,
        debit_asset,
        credit_stamp,
        debit_stamp,
        credit_amount,
        debit_amount,
        credit_settled,
        debit_settled,
        credit_stamp as stamp
      from
        transactions
      where
        credit_settled = true
        and credit_account = any ($1)
        and debit_account != all ($1)
    )
    union all
    -- ins
    (
      select
        id,
        note,
        credit_account,
        debit_account,
        credit_asset,
        debit_asset,
        credit_stamp,
        debit_stamp,
        credit_amount,
        debit_amount,
        credit_settled,
        debit_settled,
        debit_stamp as stamp
      from
        transactions
      where
        debit_settled = true
        and credit_account != all ($1)
        and debit_account = any ($1)
    )
    -- transfers
    union all
    (
      select
        id,
        note,
        credit_account,
        debit_account,
        credit_asset,
        debit_asset,
        credit_stamp,
        debit_stamp,
        credit_amount,
        debit_amount,
        credit_settled,
        debit_settled,
        debit_stamp as stamp
      from
        transactions
      where
        debit_settled = true
        and credit_account = any ($1)
        and debit_account = any ($1)
    )
  )