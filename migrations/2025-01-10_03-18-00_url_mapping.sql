CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table public.apis (
    id uuid default uuid_generate_v4() primary key,
    url varchar not null unique,
    route varchar not null unique,
    active boolean not null default false,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);