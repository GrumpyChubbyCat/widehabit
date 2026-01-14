// @generated automatically by Diesel CLI.

diesel::table! {
    habit_logs (habit_log_id) {
        habit_log_id -> Int8,
        habit_id -> Uuid,
        habit_schedule_id -> Nullable<Uuid>,
        log_date -> Date,
        actual_start -> Nullable<Timestamptz>,
        actual_end -> Nullable<Timestamptz>,
        comment -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    habit_schedules (habit_schedule_id) {
        habit_schedule_id -> Uuid,
        habit_id -> Uuid,
        day_of_week -> Int2,
        start_time -> Time,
        end_time -> Time,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        version_id -> Uuid,
    }
}

diesel::table! {
    habit_statuses (habit_status_id) {
        habit_status_id -> Int4,
        #[max_length = 50]
        name -> Varchar,
    }
}

diesel::table! {
    habits (habit_id) {
        habit_id -> Uuid,
        user_id -> Uuid,
        habit_status_id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        about -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    roles (role_id) {
        role_id -> Int4,
        title -> Varchar,
        about -> Text,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        refresh_hash -> Nullable<Varchar>,
        role_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(habit_logs -> habit_schedules (habit_schedule_id));
diesel::joinable!(habit_logs -> habits (habit_id));
diesel::joinable!(habit_schedules -> habits (habit_id));
diesel::joinable!(habits -> habit_statuses (habit_status_id));
diesel::joinable!(habits -> users (user_id));
diesel::joinable!(users -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    habit_logs,
    habit_schedules,
    habit_statuses,
    habits,
    roles,
    users,
);
