begin;

-- create asset type enum
create type asset_type as enum('currency');

-- create assets table
create table if not exists
    assets (
        id varchar(21) primary key,
        ticker varchar(8) not null unique,
        symbol varchar(8),
        label varchar(255) not null,
        precision smallint not null default 0 check (
            precision >= 0
            and precision <= 4
        ),
        atype asset_type not null
    );

-- insert default currencies
insert into
    assets (
        id,
        ticker,
        symbol,
        label,
        precision,
        atype
    )
values
    (
        'EWBwPFvyfRaTPChAoSAP7',
        'eur',
        '€',
        'Euro',
        2,
        'currency'
    ),
    (
        'UtqSw6AWO3nSYCgjiqEjR',
        'usd',
        '$',
        'U.S. dollar',
        2,
        'currency'
    ),
    (
        'SM6v1TwJkIEm9qkDOvjJZ',
        'jpy',
        '¥',
        'Japanese yen',
        0,
        'currency'
    );

commit;