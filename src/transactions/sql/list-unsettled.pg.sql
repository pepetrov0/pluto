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
  debit_settled
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
        credit_stamp as stamp,
        seq
      from
        transactions
      where
        credit_settled = false
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
        debit_stamp as stamp,
        seq
      from
        transactions
      where
        debit_settled = false
        and credit_account != all ($1)
        and debit_account = any ($1)
    )
    union all
    -- transfers
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
        debit_stamp as stamp,
        seq
      from
        transactions
      where
        debit_settled = false
        and credit_account = any ($1)
        and debit_account = any ($1)
    )
  )
order by
  stamp asc,
  seq asc