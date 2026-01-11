-- This file should undo anything in `up.sql`
ALTER TABLE users
ALTER COLUMN role_id
DROP DEFAULT;