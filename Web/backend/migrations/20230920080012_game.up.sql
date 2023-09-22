-- Add up migration script here
create table public.game
(
    version      bigint                    not null
        constraint game_pk
            primary key,
    description  text,
    installed_on timestamptz default now() not null,
    is_live      boolean                   not null
);

alter table public.game
    owner to admin;
