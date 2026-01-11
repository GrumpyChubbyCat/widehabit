-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS trg_users_updated_at ON users;

ALTER TABLE users
DROP COLUMN IF EXISTS updated_at;

ALTER TABLE users
DROP COLUMN IF EXISTS created_at;

-- We keep the function because it might be needed for other migrations