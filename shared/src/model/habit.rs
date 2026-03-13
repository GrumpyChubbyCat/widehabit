use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(ToSchema, Serialize, Deserialize, Clone)]
pub enum HabitStatus {
    Progress,
    Mastered,
    Completed
}

impl From<i32> for HabitStatus {
    fn from(v: i32) -> Self {
        match v {
            1 => HabitStatus::Progress,
            2 => HabitStatus::Mastered,
            _ => HabitStatus::Completed
        }
    }
}

#[derive(ToSchema, Serialize, Deserialize, Clone)]
pub struct HabitData {
    pub habit_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: HabitStatus
}

#[derive(ToSchema, Serialize, Deserialize)]
pub struct NewHabitReq {
    pub name: String,
    pub description: Option<String>
}

#[derive(ToSchema, Serialize, Deserialize)]
pub struct UpdateHabitRes {
    pub habit_id: Uuid,
    pub name: String,
    pub description: Option<String>
}