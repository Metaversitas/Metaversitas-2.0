-- Add up migration script here
create table public.game (
                             version bigint primary key not null,
                             description text,
                             installed_on timestamp with time zone not null default now(),
                             is_live boolean not null,
                             updated_at timestamp with time zone not null default now(),
                             created_at timestamp with time zone not null default now()
);

alter table public.game
    owner to admin;
