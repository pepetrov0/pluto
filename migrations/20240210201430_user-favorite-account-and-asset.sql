alter table if exists users
add column if not exists favorite_asset varchar(21) not null references assets (id) on update cascade on delete restrict;

alter table if exists users
add column if not exists favorite_account varchar(21) not null references accounts (id) on update cascade on delete restrict;