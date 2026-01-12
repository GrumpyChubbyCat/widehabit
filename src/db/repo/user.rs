use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    db::{
        DbPool,
        entity::{NewUser, User},
        schema::users,
    },
    errors::InternalError,
};

pub struct UserRepository {
    db_pool: DbPool,
}

impl UserRepository {
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }

    pub async fn create(&self, new_user: NewUser<'_>) -> Result<(), InternalError> {
        let mut conn = self.db_pool.get().await?;

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(&mut conn)
            .await
            .map_err(|e| {
                if let diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) = e
                {
                    tracing::warn!(
                        email = new_user.email,
                        user_name = new_user.username,
                        "registration_failed_duplicate"
                    );
                    return InternalError::AlreadyExists; // 409 Conflict
                }

                tracing::error!(error = ?e, "database_error_during_registration");
                InternalError::from(e)
            })?;
        Ok(())
    }

    pub async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<User>, InternalError> {
        let mut conn = self.db_pool.get().await?;

        let user = User::query()
            .filter(users::user_id.eq(&user_id))
            .first::<User>(&mut conn)
            .await
            .optional()?;

        Ok(user)
    }

    pub async fn find_by_user_name(&self, username: &str) -> Result<Option<User>, InternalError> {
        let mut conn = self.db_pool.get().await?;

        let user = User::query()
            .filter(users::username.eq(&username))
            .first::<User>(&mut conn)
            .await
            .optional()?;

        Ok(user)
    }

    pub async fn update_refresh_token(
        &self,
        user_id: Uuid,
        hashed_token: &str,
    ) -> Result<(), InternalError> {
        let mut conn = self.db_pool.get().await?;
        diesel::update(users::table.filter(users::user_id.eq(user_id)))
            .set(users::refresh_hash.eq(hashed_token))
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn delete_refresh_token(&self, user_id: Uuid) -> Result<(), InternalError> {
        let mut conn = self.db_pool.get().await?;
        diesel::update(users::table.filter(users::user_id.eq(user_id)))
            .set(users::refresh_hash.eq(None::<String>))
            .execute(&mut conn)
            .await?;

        Ok(())
    }
}