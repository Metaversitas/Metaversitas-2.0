-- Add up migration script here
-- Add migration script here

CREATE DOMAIN user_role AS text CHECK (VALUE IN ('administrator', 'dosen', 'mahasiswa'));
create domain email_address as text constraint valid_email_format check (value ~* '^(?:[^<>()\[\]\\.,;:\s@"]+(?:\.[^<>()\[\]\\.,;:\s@"]+)*|".+")@((?:\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(?:[a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,})$');
create domain ethereum_addresses as text constraint valid_ethereum_address check (value ~* '^0x[a-fA-F0-9]{40}$'::text);
alter domain ethereum_addresses owner to admin;
alter domain email_address owner to admin;
alter domain user_role owner to admin;

create table if not exists blockchain_authentication
(
    blockchain_auth_id uuid default gen_random_uuid() not null
        constraint blockchain_external_authentication_id_key
            unique,
    public_address     ethereum_addresses             not null
        unique
);
alter table blockchain_authentication owner to admin;
create unique index if not exists blockchain_external_authentication_pkey on blockchain_authentication (blockchain_auth_id);

create table if not exists external_authentication
(
    external_authentication_id uuid default gen_random_uuid() not null
        primary key
        unique,
    blockchain_auth_id         uuid
        unique
        constraint external_authentication_blockchain_authentication_null_fk
            references blockchain_authentication (blockchain_auth_id)
);

create table if not exists users
(
    user_id                    uuid                     default gen_random_uuid() not null
        primary key
        unique,
    email                      email_address                                      not null
        unique,
    password_hash              text                                               not null,
    nickname                   text,
    is_verified                boolean                                            not null,
    created_at                 timestamp with time zone default now()             not null,
    role                       user_role                                          not null,
    external_authentication_id uuid
        unique
        constraint users_external_authentication_null_fk
            references external_authentication,
    updated_at                 timestamp with time zone default now()             not null
);

alter table users owner to admin;