begin;

-- create sessions table
create table if not exists
    sessions (
        id varchar(21) primary key,
        usr varchar(255) not null
    );

commit;