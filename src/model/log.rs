use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct NewHabitLogReq {
    pub habit_schedule_id: Option<Uuid>,
    pub log_date: Option<NaiveDate>,
    pub actual_start: Option<DateTime<Utc>>,
    pub actual_end: Option<DateTime<Utc>>,
    pub comment: Option<String>
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HabitLogData {
    pub habit_log_id: Uuid,
    pub habit_id: Uuid,
    pub habit_schedule_id: Option<Uuid>,
    pub log_date: NaiveDate,
    pub actual_start: Option<DateTime<Utc>>,
    pub actual_end: Option<DateTime<Utc>>,
    pub comment: Option<String>
}