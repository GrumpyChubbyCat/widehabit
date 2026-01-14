use crate::db::schema::{habit_schedules, habits, users};
use chrono::{DateTime, NaiveTime, Utc};
use diesel::{HasQuery, Insertable};
use uuid::Uuid;

pub struct CountedEntities<T> {
    pub entities: Vec<T>,
    pub total_count: i64
}

#[derive(HasQuery, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub refresh_hash: Option<String>,
    pub role_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, PartialEq, Debug)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
}

#[derive(HasQuery, Debug)]
#[diesel(table_name = habits)]
pub struct Habit {
    pub habit_id: Uuid,
    pub user_id: Uuid,
    pub habit_status_id: i32,
    pub title: String,
    pub about: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, PartialEq, Debug)]
#[diesel(table_name = habits)]
pub struct NewHabit<'a> {
    pub user_id: Uuid,
    pub title: &'a str,
    pub about: Option<&'a str>,
}

#[derive(HasQuery, Debug)]
#[diesel(table_name = habit_schedules)]
pub struct HabitSchedule {
    pub habit_schedule_id: Uuid,
    pub habit_id: Uuid,
    pub day_of_week: i16,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version_id: Uuid
}

#[derive(Debug, Insertable)]
#[diesel(table_name = habit_schedules)]
pub struct NewHabitSchedule {
    pub habit_id: Uuid,
    pub version_id: Uuid,
    pub day_of_week: i16,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}