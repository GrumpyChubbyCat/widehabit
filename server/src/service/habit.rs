use uuid::Uuid;

use shared::model::{
    PagedResponse,
    habit::{HabitData, HabitStatus, NewHabitReq},
};

use crate::{
    db::{entity::NewHabit, repo::habit::HabitRepository},
    errors::InternalError,
};

pub struct HabitService {
    habit_repo: HabitRepository,
}

impl HabitService {
    pub fn new(habit_repo: HabitRepository) -> Self {
        Self { habit_repo }
    }

    pub async fn get_by_id(
        &self,
        habit_id: Uuid,
        user_id: Uuid,
    ) -> Result<HabitData, InternalError> {
        let habit = self
            .habit_repo
            .find_by_habit_id(habit_id, user_id)
            .await?
            .ok_or(InternalError::NotFound)?;

        Ok(HabitData {
            habit_id,
            name: habit.title,
            description: habit.about,
            status: HabitStatus::from(habit.habit_status_id),
        })
    }

    pub async fn get_paged(
        &self,
        user_id: Uuid,
        page: i64,
        page_size: i64,
    ) -> Result<PagedResponse<HabitData>, InternalError> {
        let counted_habits = self.habit_repo.find_paged(user_id, page, page_size).await?;

        let habits_data = counted_habits
            .entities
            .into_iter()
            .map(|habit| HabitData {
                habit_id: habit.habit_id,
                name: habit.title,
                description: habit.about,
                status: HabitStatus::from(habit.habit_status_id),
            })
            .collect();

        Ok(PagedResponse::<HabitData> {
            items: habits_data,
            total_count: counted_habits.total_count,
            page,
            page_size,
        })
    }

    pub async fn create(
        &self,
        habit_data: NewHabitReq,
        user_id: Uuid,
    ) -> Result<HabitData, InternalError> {
        let new_habit = NewHabit {
            user_id: user_id,
            title: &habit_data.name,
            about: habit_data.description.as_deref(),
        };

        let created_habit = self.habit_repo.create(new_habit).await?;
        let habit_data = HabitData {
            habit_id: created_habit.habit_id,
            name: created_habit.title,
            description: created_habit.about,
            status: HabitStatus::from(created_habit.habit_status_id),
        };

        Ok(habit_data)
    }

    pub async fn update(
        &self,
        habit_id: Uuid,
        user_id: Uuid,
        habit_data: &NewHabitReq,
    ) -> Result<(), InternalError> {
        self.habit_repo
            .update(
                habit_id,
                user_id,
                &habit_data.name,
                habit_data.description.as_deref(),
            )
            .await
    }

    pub async fn delete(&self, habit_id: Uuid, user_id: Uuid) -> Result<(), InternalError> {
        self.habit_repo.delete(habit_id, user_id).await
    }
}
