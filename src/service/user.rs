use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use uuid::Uuid;

use crate::{
    config::AuthConfig, db::repo::UserRepository, errors::InternalError, model::{
        auth::{AccessClaims, RefreshClaims},
        user::{UserAuthReq, UserRole, UserRoleData},
    }
};

pub struct UserService {
    user_repo: UserRepository,
    jwt_secret: String,
    access_lt: i64,     // Access token lifetime
    refresh_lt: i64,    // Refresh token lifetime
}

impl UserService {
    pub fn new(
        user_repo: UserRepository,
        auth_config: AuthConfig,
    ) -> Self {
        Self {
            user_repo,
            jwt_secret: auth_config.jwt_secret,
            access_lt: auth_config.access_lt,
            refresh_lt: auth_config.refresh_lt
        }
    }

    pub async fn authenticate(
        &self,
        user_info: UserAuthReq,
    ) -> Result<UserRoleData, InternalError> {
        let user = self
            .user_repo
            .find_by_user_name(&user_info.username)
            .await?
            .ok_or(InternalError::InvalidCredentials)?;

        self.verify_password(user_info.password, user.password_hash)
            .await?
            .then_some(()) // cast bool to Option
            .ok_or(InternalError::InvalidCredentials)?;

        Ok(UserRoleData {
            user_id: user.user_id,
            role: UserRole::from(user.role_id),
        })
    }

    async fn verify_password(
        &self,
        plain_password: String,
        hashed_password: String,
    ) -> Result<bool, InternalError> {
        tokio::task::spawn_blocking(move || {
            let parsed_hash = PasswordHash::new(&hashed_password).map_err(|e| {
                tracing::error!("Internal error: invalid hash format in DB: {}", e);
                InternalError::HashError(e.to_string())
            })?;

            let is_valid = Argon2::default()
                .verify_password(plain_password.as_bytes(), &parsed_hash)
                .is_ok();

            Ok(is_valid)
        })
        .await
        .map_err(|_| InternalError::TokioThreadPoolError("Execution error".to_string()))?
    }

    pub async fn create_access_token(
        &self,
        user_id: Uuid,
        role_id: i32,
    ) -> Result<String, InternalError> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::minutes(self.access_lt))
            .expect("valid timestamp")
            .timestamp();

        let claims = AccessClaims {
            sub: user_id,
            role: role_id,
            exp: expiration,
            iat: Utc::now().timestamp(),
        };

        let encoded_jwt = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| {
            let err_msg = format!("JWT error: {}", &e);
            tracing::error!(err_msg);
            InternalError::JWTError(err_msg)
        });

        encoded_jwt
    }

    async fn hash_token_blocking(&self, token: String) -> Result<String, InternalError> {
        tokio::task::spawn_blocking(move || {
            let salt = SaltString::generate(&mut OsRng);

            Argon2::default()
                .hash_password(token.as_bytes(), &salt)
                .map(|h| h.to_string())
                .map_err(|e| InternalError::HashError(e.to_string()))
        })
        .await
        .map_err(|_| InternalError::TokioThreadPoolError("Execution error".to_string()))?
    }

    pub async fn create_refresh_token(&self, user_id: Uuid) -> Result<String, InternalError> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(self.refresh_lt)) // Lives an hours
            .expect("valid timestamp")
            .timestamp();

        let claims = RefreshClaims {
            sub: user_id,
            exp: expiration,
            iat: Utc::now().timestamp(),
            jti: Uuid::new_v4(),
        };

        // Generate raw jwt refresh token
        let raw_token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| {
            let err_msg = format!("JWT error: {}", &e);
            tracing::error!(err_msg);
            InternalError::JWTError(err_msg)
        })?;

        // Hash token for db saving
        let hashed_token = self.hash_token_blocking(raw_token.clone()).await?;

        // Save token to db
        self.user_repo
            .update_refresh_token(user_id, &hashed_token)
            .await?;

        Ok(raw_token)
    }
}
