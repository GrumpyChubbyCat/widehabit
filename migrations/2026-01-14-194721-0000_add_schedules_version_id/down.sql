-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_habit_schedules_version_id;

ALTER TABLE habit_schedules
DROP COLUMN IF EXISTS version_id;