use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

pub mod auth;
pub mod habit;
pub mod schedule;
pub mod user;
pub mod log;

#[derive(ToSchema, Serialize)]
pub struct PagedResponse<T> {
    pub items: Vec<T>,
    pub total_count: i64,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Deserialize, IntoParams)]
pub struct PaginationParams {
    pub page: i64,
    pub limit: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum DayOfWeek {
    Monday = 0,
    Tuesday = 1,
    Wednesday = 2,
    Thursday = 3,
    Friday = 4,
    Saturday = 5,
    Sunday = 6,
}

impl TryFrom<i16> for DayOfWeek {
    type Error = ();
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(DayOfWeek::Monday),
            1 => Ok(DayOfWeek::Tuesday),
            2 => Ok(DayOfWeek::Wednesday),
            3 => Ok(DayOfWeek::Thursday),
            4 => Ok(DayOfWeek::Friday),
            5 => Ok(DayOfWeek::Saturday),
            6 => Ok(DayOfWeek::Sunday),
            _ => Err(()),
        }
    }
}

impl From<DayOfWeek> for i16 {
    fn from(day: DayOfWeek) -> Self {
        day as i16
    }
}