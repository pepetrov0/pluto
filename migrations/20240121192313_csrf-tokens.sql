-- create csrf_tokens table
create table if not exists
  csrf_tokens (
    id varchar(21) primary key,
    usr varchar(255) not null,
    usage varchar(128) not null,
    stamp timestamp not null default current_timestamp
  );

create index if not exists stamp_idx on csrf_tokens (stamp);

create or replace view
  valid_csrf_tokens (id, usr, usage) as
select
  id,
  usr,
  usage
from
  csrf_tokens
where
  stamp > current_timestamp - '30 mins'::interval;

create or replace view
  invalid_csrf_tokens (id, usr, usage) as
select
  id,
  usr,
  usage
from
  csrf_tokens
where
  stamp < current_timestamp - '30 mins'::interval;

create
or replace function cleanup_csrf_tokens_fn () returns trigger as $$
begin
  delete from invalid_csrf_tokens;
  return NEW;
end; 
$$ language plpgsql;

create
or replace trigger cleanup_csrf_tokens
after insert on csrf_tokens
execute function cleanup_csrf_tokens_fn ();