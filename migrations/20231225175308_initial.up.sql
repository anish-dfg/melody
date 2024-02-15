-- Add up migration script here
begin;
--
-- create uuid extension
create extension if not exists "uuid-ossp";
--
-- update timestamp function
create or replace function update_timestamp()
  returns trigger
  as $$
begin
  new.updated_at = current_timestamp;
  return new;
end;
$$
language plpgsql;
--
-- auth_provider type
create type auth_provider as enum (
  'okta',
  'auth0',
  'keycloak',
  'developforgood'
);
--
-- hash_algorithm type
create type hash_algorithm as enum(
  'argon2',
  'bcrypt',
  'none'
);
--
-- users table
create table if not exists users(
  id uuid not null default uuid_generate_v4() primary key,
  first_name text not null,
  last_name text not null,
  email text not null,
  username text not null,
  image_uri text not null,
  auth_provider auth_provider not null default 'auth0'::auth_provider,
  created_at timestamptz not null default current_timestamp,
  updated_at timestamptz not null default current_timestamp,
  unique (email)
);
create or replace trigger update_users_timestamp
  before update on users for each row
  execute function update_timestamp();
--
-- metrics
commit;

