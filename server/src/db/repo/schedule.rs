use diesel::prelude::*;
use diesel_async::{AsyncConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    db::{
        DbPool,
        entity::{HabitSchedule, NewHabitSchedule},
        schema::{habit_schedules, habits},
    },
    errors::InternalError,
};

pub struct HabitScheduleRepository {
    db_pool: DbPool,
}

impl HabitScheduleRepository {
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }

    pub async fn get_all(&self, user_id: Uuid) -> Result<Vec<HabitSchedule>, InternalError> {
        let mut conn = self.db_pool.get().await?;

        let items = habit_schedules::table
            .inner_join(habits::table)
            .filter(habits::user_id.eq(user_id))
            .filter(habit_schedules::is_active.eq(true))
            .select(habit_schedules::all_columns)
            .order((
                habit_schedules::day_of_week.asc(),
                habit_schedules::start_time.asc(),
            ))
            .get_results::<HabitSchedule>(&mut conn)
            .await?;
        Ok(items)
    }

    pub async fn get(
        &self,
        habit_id: Uuid,
        user_id: Uuid,
    ) -> Result<Vec<HabitSchedule>, InternalError> {
        let mut conn = self.db_pool.get().await?;

        conn.transaction::<Vec<HabitSchedule>, InternalError, _>(|conn| {
            Box::pin(async move {
                let count = habits::table
                    .filter(habits::habit_id.eq(habit_id))
                    .filter(habits::user_id.eq(user_id))
                    .execute(conn)
                    .await?;

                if count == 0 {
                    return Err(InternalError::NotFound);
                }

                let items = HabitSchedule::query()
                    .filter(habit_schedules::habit_id.eq(habit_id))
                    .filter(habit_schedules::is_active.eq(true))
                    .order((
                        habit_schedules::day_of_week.asc(),
                        habit_schedules::start_time.asc(),
                    ))
                    .get_results::<HabitSchedule>(conn)
                    .await?;
                Ok(items)
            })
        })
        .await
    }

    pub async fn update(
        &self,
        habit_id: Uuid,
        user_id: Uuid,
        new_plans: Vec<NewHabitSchedule>,
    ) -> Result<Vec<HabitSchedule>, InternalError> {
        let mut conn = self.db_pool.get().await?;

        let mut inserted_items = conn
            .transaction::<Vec<HabitSchedule>, InternalError, _>(|conn| {
                Box::pin(async move {
                    let count = habits::table
                        .filter(habits::habit_id.eq(habit_id))
                        .filter(habits::user_id.eq(user_id))
                        .execute(conn)
                        .await?;

                    if count == 0 {
                        return Err(InternalError::NotFound);
                    }

                    diesel::update(
                        habit_schedules::table.filter(habit_schedules::habit_id.eq(habit_id)),
                    )
                    .set(habit_schedules::is_active.eq(false))
                    .execute(conn)
                    .await?;

                    if !new_plans.is_empty() {
                        let rows = diesel::insert_into(habit_schedules::table)
                            .values(&new_plans)
                            .get_results::<HabitSchedule>(conn)
                            .await?;
                        Ok(rows)
                    } else {
                        Ok(vec![])
                    }
                })
            })
            .await?;

        inserted_items.sort_by_key(|item| item.day_of_week);

        Ok(inserted_items)
    }
}
