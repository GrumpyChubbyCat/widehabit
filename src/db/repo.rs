use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    db::{DbPool, entity::User, schema::users},
    errors::InternalError,
};

pub struct UserRepository {
    db_pool: DbPool,
}

impl UserRepository {
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
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
