use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    db::{
        DbPool,
        entity::{CountedEntities, Habit, NewHabit},
        schema::habits,
    },
    errors::InternalError,
};

pub struct HabitRepository {
    db_pool: DbPool,
}

impl HabitRepository {
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }

    pub async fn create(&self, new_habit: NewHabit<'_>) -> Result<(), InternalError> {
        let mut conn = self.db_pool.get().await?;

        diesel::insert_into(habits::table)
            .values(&new_habit)
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn find_by_habit_id(
        &self,
        habit_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<Habit>, InternalError> {
        let mut conn = self.db_pool.get().await?;

        let habit = Habit::query()
            .filter(habits::habit_id.eq(&habit_id))
            .filter(habits::user_id.eq(user_id))
            .first::<Habit>(&mut conn)
            .await
            .optional()?;

        Ok(habit)
    }

    pub async fn find_paged(
        &self,
        user_id: Uuid,
        page: i64,
        limit: i64,
    ) -> Result<CountedEntities<Habit>, InternalError> {
        let mut conn = self.db_pool.get().await?;

        let safe_page = if page < 1 { 1 } else { page };
        let offset = (safe_page - 1) * limit;
        let habits = habits::table
            .filter(habits::user_id.eq(user_id))
            .order(habits::created_at.desc())
            .limit(limit)
            .offset(offset) // Вот теперь тут будет 0 для первой страницы
            .load::<Habit>(&mut conn)
            .await?;

        let total_count = habits::table
            .filter(habits::user_id.eq(user_id))
            .count()
            .get_result::<i64>(&mut conn)
            .await?;

        let counted_habits = CountedEntities::<Habit> {
            entities: habits,
            total_count,
        };

        Ok(counted_habits)
    }
}
