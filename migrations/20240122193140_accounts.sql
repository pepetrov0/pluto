begin;

-- create accounts table
create table if not exists
    accounts (
        id varchar(21) primary key,
        name varchar(255) not null,
        default_asset varchar(21) references assets (id) on update cascade on delete restrict
    );

-- create accounts_ownerships table
create table if not exists
    accounts_ownerships (
        id serial primary key,
        usr varchar(21) not null references users (id) on update cascade on delete restrict,
        account varchar(21) not null references accounts (id) on update cascade on delete restrict,
        unique (usr, account)
    );

commit;