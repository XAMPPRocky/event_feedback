-- Your SQL goes here
create table feedback (
    id uuid not null primary key,
    secret varchar not null,
    body varchar not null
)
