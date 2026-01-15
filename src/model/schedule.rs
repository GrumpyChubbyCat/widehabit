use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::{Validate, ValidationError};

use crate::model::DayOfWeek;

#[derive(Debug, Deserialize, ToSchema, Validate)]
#[validate(schema(function = "validate_times"))]
pub struct ScheduleItemReq {
    pub day: DayOfWeek,
    #[schema(value_type = String, example = "08:00:00")]
    pub start_time: NaiveTime,
    #[schema(value_type = String, example = "09:00:00")]
    pub end_time: NaiveTime
}

fn validate_times(item: &ScheduleItemReq) -> Result<(), ValidationError> {
    if item.start_time >= item.end_time {
        return Err(ValidationError::new("invalid_time_range")
            .with_message("Start time must be strictly before end time".into()));
    }
    Ok(())
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct SetScheduleReq {
    pub habit_id: Uuid,
    #[validate(nested)]
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
pub struct ScheduleRes {
    pub schedules: Vec<ScheduleItemRes>,
}