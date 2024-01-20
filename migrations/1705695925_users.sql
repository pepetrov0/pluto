begin;

-- enable pgcrypt
create extension if not exists pgcrypto;

-- create users table
create table if not exists
    users (
        id varchar(21) primary key,
        email varchar(255) not null unique,
        password varchar(128)
    );

commit;