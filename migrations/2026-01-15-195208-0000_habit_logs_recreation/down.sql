-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS habit_logs;

CREATE TABLE
    habit_logs (
        habit_log_id BIGSERIAL PRIMARY KEY,
        habit_id UUID NOT NULL REFERENCES habits (habit_id) ON DELETE CASCADE,
        habit_schedule_id UUID REFERENCES habit_schedules (habit_schedule_id) ON DELETE SET NULL,
        log_date DATE NOT NULL DEFAULT CURRENT_DATE,
        actual_start TIMESTAMPTZ,
        actual_end TIMESTAMPTZ,
        comment TEXT,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        CONSTRAINT actual_start_before_end CHECK (actual_start < actual_end)
    );

CREATE TRIGGER trg_habit_logs_updated_at BEFORE 
UPDATE ON habit_logs FOR EACH ROW EXECUTE FUNCTION set_updated_at();

CREATE INDEX idx_habit_logs_date ON habit_logs (log_date);

CREATE INDEX idx_habit_logs_schedule ON habit_logs (habit_schedule_id);