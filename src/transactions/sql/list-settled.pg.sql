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
    select distinct
      on (id) id,
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
      stamp
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
            and credit_account = any ($3)
            and debit_account != any ($3)
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
            and credit_account != any ($3)
            and debit_account = any ($3)
        )
      )
    order by
      id,
      stamp desc
  )
order by
  stamp desc
offset
  $1
limit
  $2