create table "user"
(
  user_id uuid primary key default gen_random_uuid(),

  username text collate "case_insensitive" unique not null,

  email text collate "case_insensitive" unique not null,
  is_email_verified boolean not null default false,

  password_hash text not null,

  created_at timestamptz not null default now(),
  updated_at timestamptz
);

SELECT trigger_updated_at('"user"');
