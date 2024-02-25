-- create sessions table
create table if not exists
  sessions (
    id varchar(21) primary key,
    usr varchar(21) not null references users (id) on update cascade on delete cascade
  );