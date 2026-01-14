-- Your SQL goes here
ALTER TABLE habit_schedules
ADD COLUMN version_id UUID NOT NULL DEFAULT uuid_generate_v4 ();

-- Index for fast select
CREATE INDEX idx_habit_schedules_version_id ON habit_schedules (version_id);