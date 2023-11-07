-- Add down migration script here
alter table public.users_identity
    drop column photo_url;