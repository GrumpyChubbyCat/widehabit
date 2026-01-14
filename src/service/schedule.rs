use uuid::Uuid;

use crate::{
    db::{entity::NewHabitSchedule, repo::schedule::HabitScheduleRepository},
    errors::InternalError,
    model::{
        DayOfWeek,
        schedule::{ScheduleItemReq, ScheduleItemRes},
    },
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

    pub async fn update_schedule(
        &self,
        habit_id: Uuid,
        user_id: Uuid,
        schedule_items: Vec<ScheduleItemReq>,
    ) -> Result<Vec<ScheduleItemRes>, InternalError> {
        let new_plans: Vec<NewHabitSchedule> = schedule_items
            .into_iter()
            .map(|item| NewHabitSchedule {
                habit_id,
                day_of_week: item.day.into(),
                start_time: item.start_time,
                end_time: item.end_time,
            })
            .collect();

        let new_plans = self
            .habit_schedule_repo
            .update_schedule(habit_id, user_id, new_plans)
            .await?;

        let new_plans_res: Vec<ScheduleItemRes> = new_plans
            .into_iter()
            .map(|schedule_item| {
                Ok(ScheduleItemRes {
                    schedule_id: schedule_item.habit_schedule_id,
                    habit_id: schedule_item.habit_id,
                    version_id: schedule_item.version_id,
                    day: DayOfWeek::try_from(schedule_item.day_of_week)
                        .map_err(|_| InternalError::Cast("Cant cast i16 to DayOfWeel".into()))?,
                    start_time: schedule_item.start_time,
                    end_time: schedule_item.end_time,
                    created_at: schedule_item.created_at,
                })
            })
            .collect::<Result<Vec<_>, InternalError>>()?;

        Ok(new_plans_res)
    }
}
