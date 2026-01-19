use chrono::Utc;
use uuid::Uuid;

use shared::model::{
    PagedResponse,
    log::{HabitLogData, NewHabitLogReq},
};

use crate::{
    db::{entity::NewHabitLog, repo::log::HabitLogRepository},
    errors::InternalError,
};

pub struct HabitLogService {
    habit_log_repo: HabitLogRepository,
}

impl HabitLogService {
    pub fn new(habit_log_repo: HabitLogRepository) -> Self {
        Self { habit_log_repo }
    }

    pub async fn get_paged(
        &self,
        habit_id: Uuid,
        user_id: Uuid,
        page: i64,
        page_size: i64,
    ) -> Result<PagedResponse<HabitLogData>, InternalError> {
        let counted_habit_logs = self
            .habit_log_repo
            .find_paged(habit_id, user_id, page, page_size)
            .await?;

        let habit_logs_data = counted_habit_logs
            .entities
            .into_iter()
            .map(|habit_log| HabitLogData {
                habit_log_id: habit_log.habit_log_id,
                habit_id: habit_log.habit_id,
                habit_schedule_id: habit_log.habit_schedule_id,
                log_date: habit_log.log_date,
                actual_start: habit_log.actual_start,
                actual_end: habit_log.actual_end,
                comment: habit_log.comment,
            })
            .collect();

        Ok(PagedResponse::<HabitLogData> {
            items: habit_logs_data,
            total_count: counted_habit_logs.total_count,
            page,
            page_size,
        })
    }

    pub async fn create(
        &self,
        habit_id: Uuid,
        user_id: Uuid,
        new_log: NewHabitLogReq,
    ) -> Result<HabitLogData, InternalError> {
        let log_date = new_log.log_date.unwrap_or_else(|| Utc::now().date_naive());
        let new_log_db = NewHabitLog {
            habit_id,
            log_date,
            habit_schedule_id: new_log.habit_schedule_id,
            actual_start: new_log.actual_start,
            actual_end: new_log.actual_end,
            comment: new_log.comment.as_deref(),
        };

        let created_log = self
            .habit_log_repo
            .create(habit_id, user_id, new_log_db)
            .await?;

        Ok(HabitLogData {
            habit_log_id: created_log.habit_log_id,
            habit_id: created_log.habit_id,
            habit_schedule_id: created_log.habit_schedule_id,
            log_date: created_log.log_date,
            actual_start: created_log.actual_start,
            actual_end: created_log.actual_end,
            comment: created_log.comment,
        })
    }
}
