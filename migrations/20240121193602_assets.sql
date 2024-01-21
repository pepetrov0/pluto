begin;

-- create asset type enum
create type asset_type as enum('currency');

-- create assets table
create table if not exists
    assets (
        id varchar(21) primary key,
        symbol varchar(8) not null unique,
        label varchar(255) not null,
        precision integer not null default 0 check (
            precision >= 0
            and precision <= 4
        ),
        atype asset_type not null
    );

-- insert default currencies
insert into
    assets (
        id,
        symbol,
        label,
        precision,
        type
    )
values
    (
        'EWBwPFvyfRaTPChAoSAP7',
        'eur',
        'Euro',
        2,
        'currency'
    ),
    (
        'UtqSw6AWO3nSYCgjiqEjR',
        'usd',
        'United States Dollar',
        2,
        'currency'
    ),
    (
        'SM6v1TwJkIEm9qkDOvjJZ',
        'jpy',
        'Japanese Yen',
        0,
        'currency'
    );

commit;