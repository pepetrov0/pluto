insert into
  transactions (
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
  )
values
  ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
returning
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