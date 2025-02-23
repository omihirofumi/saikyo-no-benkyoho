-- Add migration script here
CREATE TABLE record (
  id uuid primary key default gen_random_uuid(),
  category varchar(255) not null,
  title varchar(255) not null,
  contents text not null,
  created_at timestamp not null default now()
)
