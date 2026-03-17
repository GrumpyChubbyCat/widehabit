use diesel::prelude::*;
use diesel_async::{AsyncConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{
    db::{
        DbPool,
        entity::{CountedEntities, HabitLog, NewHabitLog},
        schema::{habit_logs, habit_schedules, habits},
    },
    errors::InternalError,
};

pub struct HabitLogRepository {
    db_pool: DbPool,
}

impl HabitLogRepository {
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }

    pub async fn create(
        &self,
        habit_id: Uuid,
        user_id: Uuid,
        new_log: NewHabitLog<'_>,
    ) -> Result<HabitLog, InternalError> {
        let mut conn = self.db_pool.get().await?;

        conn.transaction::<HabitLog, InternalError, _>(|conn| {
            Box::pin(async move {
                let count = habits::table
                    .filter(habits::habit_id.eq(habit_id))
                    .filter(habits::user_id.eq(user_id))
                    .execute(conn)
                    .await?;

                if count == 0 {
                    return Err(InternalError::NotFound);
                }

                if let Some(s_id) = new_log.habit_schedule_id {
                    let schedule_valid = habit_schedules::table
                        .filter(habit_schedules::habit_schedule_id.eq(s_id))
                        .filter(habit_schedules::habit_id.eq(new_log.habit_id))
                        .execute(conn)
                        .await?;

                    if schedule_valid == 0 {
                        return Err(InternalError::BadRequest(
                            "Slot does not belongs to habit".into(),
                        ));
                    }
                }

                let created_habit_log = diesel::insert_into(habit_logs::table)
                    .values(&new_log)
                    .get_result::<HabitLog>(conn)
                    .await?;

                Ok(created_habit_log)
            })
        })
        .await
    }

    pub async fn find_paged(
        &self,
        habit_id: Uuid,
        user_id: Uuid,
        page: i64,
        limit: i64,
    ) -> Result<CountedEntities<HabitLog>, InternalError> {
        let mut conn = self.db_pool.get().await?;

        conn.transaction::<CountedEntities<HabitLog>, InternalError, _>(|conn| {
            let safe_page = if page < 1 { 1 } else { page }; // Normalization for safety
            let offset = (safe_page - 1) * limit;

            Box::pin(async move {
                let count = habits::table
                    .filter(habits::habit_id.eq(habit_id))
                    .filter(habits::user_id.eq(user_id))
                    .execute(conn)
                    .await?;

                if count == 0 {
                    return Err(InternalError::NotFound);
                }

                let habit_logs = habit_logs::table
                    .filter(habit_logs::habit_id.eq(habit_id))
                    .order(habit_logs::created_at.desc())
                    .limit(limit)
                    .offset(offset)
                    .load::<HabitLog>(conn)
                    .await?;

                let total_count = habit_logs::table
                    .filter(habit_logs::habit_id.eq(habit_id))
                    .count()
                    .get_result::<i64>(conn)
                    .await?;

                let counted_habits = CountedEntities::<HabitLog> {
                    entities: habit_logs,
                    total_count,
                };
                Ok(counted_habits)
            })
        })
        .await
    }

    pub async fn get_total_minutes(
        &self,
        habit_id: Uuid,
        user_id: Uuid,
    ) -> Result<i64, InternalError> {
        let mut conn = self.db_pool.get().await?;

        // Check ownership
        let count = habits::table
            .filter(habits::habit_id.eq(habit_id))
            .filter(habits::user_id.eq(user_id))
            .count()
            .get_result::<i64>(&mut conn)
            .await?;

        if count == 0 {
            return Err(InternalError::NotFound);
        }

        let logs = habit_logs::table
            .filter(habit_logs::habit_id.eq(habit_id))
            .filter(habit_logs::actual_start.is_not_null())
            .filter(habit_logs::actual_end.is_not_null())
            .load::<HabitLog>(&mut conn)
            .await?;

        let total_minutes = logs
            .into_iter()
            .filter_map(|log| {
                if let (Some(start), Some(end)) = (log.actual_start, log.actual_end) {
                    Some((end - start).num_minutes())
                } else {
                    None
                }
            })
            .sum();

        Ok(total_minutes)
    }
}
