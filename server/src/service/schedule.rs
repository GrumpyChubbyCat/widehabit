use uuid::Uuid;

use shared::model::{
    DayOfWeek,
    schedule::{ScheduleItemReq, ScheduleItemRes},
};

use crate::{
    db::{entity::NewHabitSchedule, repo::schedule::HabitScheduleRepository},
    errors::InternalError,
};

pub struct HabitScheduleService {
    habit_schedule_repo: HabitScheduleRepository,
}

impl HabitScheduleService {
    pub fn new(habit_schedule_repo: HabitScheduleRepository) -> Self {
        Self {
            habit_schedule_repo,
        }
    }

    pub async fn get(
        &self,
        habit_id: Uuid,
        user_id: Uuid,
    ) -> Result<Vec<ScheduleItemRes>, InternalError> {
        let last_plans_db = self.habit_schedule_repo.get(habit_id, user_id).await?;

        last_plans_db
            .into_iter()
            .map(|schedule_item| {
                Ok(ScheduleItemRes {
                    schedule_id: schedule_item.habit_schedule_id,
                    habit_id: schedule_item.habit_id,
                    version_id: schedule_item.version_id,
                    day: DayOfWeek::try_from(schedule_item.day_of_week)
                        .map_err(|_| InternalError::Cast("Cant cast i16 to DayOfWeek".into()))?,
                    start_time: schedule_item.start_time,
                    end_time: schedule_item.end_time,
                    created_at: schedule_item.created_at,
                })
            })
            .collect()
    }

    pub async fn update(
        &self,
        habit_id: Uuid,
        user_id: Uuid,
        schedule_items: Vec<ScheduleItemReq>,
    ) -> Result<Vec<ScheduleItemRes>, InternalError> {
        let version_id = Uuid::new_v4();

        let new_plans: Vec<NewHabitSchedule> = schedule_items
            .into_iter()
            .map(|item| NewHabitSchedule {
                habit_id,
                version_id,
                day_of_week: item.day.into(),
                start_time: item.start_time,
                end_time: item.end_time,
            })
            .collect();

        let new_plans_db = self
            .habit_schedule_repo
            .update(habit_id, user_id, new_plans)
            .await?;

        new_plans_db
            .into_iter()
            .map(|schedule_item| {
                Ok(ScheduleItemRes {
                    schedule_id: schedule_item.habit_schedule_id,
                    habit_id: schedule_item.habit_id,
                    version_id: schedule_item.version_id,
                    day: DayOfWeek::try_from(schedule_item.day_of_week)
                        .map_err(|_| InternalError::Cast("Cant cast i16 to DayOfWeek".into()))?,
                    start_time: schedule_item.start_time,
                    end_time: schedule_item.end_time,
                    created_at: schedule_item.created_at,
                })
            })
            .collect()
    }
}
