begin;

-- create the transactions table
create table if not exists
    transactions (
        id varchar(21) primary key,
        note varchar(255) not null,
        credit_account varchar(21) not null references accounts (id) on update cascade on delete restrict,
        debit_account varchar(21) not null references accounts (id) on update cascade on delete restrict,
        credit_asset varchar(21) not null references assets (id) on update cascade on delete restrict,
        debit_asset varchar(21) not null references assets (id) on update cascade on delete restrict,
        credit_stamp timestamp not null,
        debit_stamp timestamp not null,
        credit_amount bigint not null check (credit_amount > 0),
        debit_amount bigint not null check (debit_amount > 0),
        credit_settled boolean not null default false,
        debit_settled boolean not null default false,
        constraint credit_stamp_le_debit_stamp_check check (credit_stamp <= debit_stamp)
    );

-- create the entries view
create or replace view
    entries (id, note, account, asset, stamp, amount, settled) as
select
    id,
    note,
    credit_account as account,
    credit_asset as asset,
    credit_stamp as stamp,
    (credit_amount * -1::bigint) as amount,
    credit_settled as settled
from
    transactions
union all
select
    id,
    note,
    debit_account as account,
    debit_asset as asset,
    debit_stamp as stamp,
    debit_amount as amount,
    debit_settled as settled
from
    transactions;

commit;