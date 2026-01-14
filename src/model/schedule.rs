use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::model::DayOfWeek;

#[derive(Debug, Deserialize, ToSchema)]
pub struct ScheduleItemReq {
    pub day: DayOfWeek,
    #[schema(value_type = String, example = "08:00:00")]
    pub start_time: NaiveTime,
    #[schema(value_type = String, example = "09:00:00")]
    pub end_time: NaiveTime
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SetScheduleReq {
    pub habit_id: Uuid,
    pub schedules: Vec<ScheduleItemReq>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ScheduleItemRes {
    pub schedule_id: Uuid,
    pub habit_id: Uuid,
    pub version_id: Uuid,
    pub day: DayOfWeek,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SetScheduleRes {
    pub schedules: Vec<ScheduleItemRes>,
}