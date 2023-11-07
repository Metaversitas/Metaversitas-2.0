-- Add up migration script here
alter table public.users_identity
    add column photo_url text not null default 'user-photo-profile/user_default.png';