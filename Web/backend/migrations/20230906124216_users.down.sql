-- Add down migration script here
drop domain if exists user_roles;
drop domain if exists email_address;
drop table if exists users;
