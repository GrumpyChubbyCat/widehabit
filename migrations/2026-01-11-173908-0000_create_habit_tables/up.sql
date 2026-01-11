-- 1. Habit status reference table (Dictionary)
CREATE TABLE
    habit_statuses (
        habit_status_id SERIAL PRIMARY KEY,
        name VARCHAR(50) NOT NULL UNIQUE
    );

INSERT INTO
    habit_statuses (name)
VALUES
    ('Progress'),
    ('Mastered'),
    ('Completed');

-- 2. Main habits table
CREATE TABLE
    habits (
        habit_id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        user_id UUID NOT NULL REFERENCES users (user_id) ON DELETE CASCADE,
        habit_status_id INT NOT NULL REFERENCES habit_statuses (habit_status_id) DEFAULT 1,
        title VARCHAR(255) NOT NULL,
        about TEXT,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );

-- 3. Habit scheduling (Versioned weekly plans)
CREATE TABLE
    habit_schedules (
        habit_schedule_id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        habit_id UUID NOT NULL REFERENCES habits (habit_id) ON DELETE CASCADE,
        day_of_week SMALLINT NOT NULL CHECK (day_of_week BETWEEN 0 AND 6),
        start_time TIME NOT NULL,
        end_time TIME NOT NULL,
        is_active BOOLEAN NOT NULL DEFAULT TRUE, -- Позволяет хранить старые версии планов
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        CONSTRAINT start_before_end CHECK (start_time < end_time)
    );

-- 4. Habit execution logs (Linked to historical plans)
CREATE TABLE
    habit_logs (
        habit_log_id BIGSERIAL PRIMARY KEY,
        habit_id UUID NOT NULL REFERENCES habits (habit_id) ON DELETE CASCADE,
        -- SET NULL for safety and light support
        habit_schedule_id UUID REFERENCES habit_schedules (habit_schedule_id) ON DELETE SET NULL,
        log_date DATE NOT NULL DEFAULT CURRENT_DATE,
        actual_start TIMESTAMPTZ,
        actual_end TIMESTAMPTZ,
        comment TEXT,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        CONSTRAINT actual_start_before_end CHECK (actual_start < actual_end)
    );

-- 5. Applying triggers for automatic updated_at
-- (Assuming set_updated_at function exists from previous user migration)
CREATE TRIGGER trg_habits_updated_at BEFORE
UPDATE ON habits FOR EACH ROW EXECUTE FUNCTION set_updated_at ();

CREATE TRIGGER trg_habit_schedules_updated_at BEFORE
UPDATE ON habit_schedules FOR EACH ROW EXECUTE FUNCTION set_updated_at ();

CREATE TRIGGER trg_habit_logs_updated_at BEFORE
UPDATE ON habit_logs FOR EACH ROW EXECUTE FUNCTION set_updated_at ();

-- 6. Indexes
CREATE INDEX idx_habit_logs_date ON habit_logs (log_date);

CREATE INDEX idx_habit_schedules_habit ON habit_schedules (habit_id);

CREATE INDEX idx_habit_logs_schedule ON habit_logs (habit_schedule_id);