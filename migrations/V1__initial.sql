create table login (
    id integer constraint login_id_pk primary key autoincrement,
    hash TEXT not null,
    email TEXT not null,
    password TEXT not null,
    state TEXT default 'ENABLED' not null
);
create unique index login_email_index on login (email);
create unique index login_hash_index on login (hash);