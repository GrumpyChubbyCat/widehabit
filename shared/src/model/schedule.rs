use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::model::DayOfWeek;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct ScheduleItemReq {
    pub day: DayOfWeek,
    #[schema(value_type = String, example = "08:00:00")]
    pub start_time: NaiveTime,
    #[schema(value_type = String, example = "09:00:00")]
    pub end_time: NaiveTime,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct SetScheduleReq {
    pub habit_id: Uuid,
    #[validate(nested)]
    pub schedules: Vec<ScheduleItemReq>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct ScheduleItemRes {
    pub schedule_id: Uuid,
    pub habit_id: Uuid,
    pub version_id: Uuid,
    pub day: DayOfWeek,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct ScheduleRes {
    pub schedules: Vec<ScheduleItemRes>,
}
