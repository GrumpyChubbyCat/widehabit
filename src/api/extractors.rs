use crate::{
    api::errors::AuthError,
    config::AuthConfig,
    model::{auth::AccessClaims, user::UserRole},
};
use axum::{
    RequestPartsExt,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use std::marker::PhantomData;

pub trait AccessLevel {
    fn required_role() -> UserRole;

    fn is_satisfied(user_role: i32) -> bool {
        let required = Self::required_role() as i32;
        // ADMIN=1, USER=2, BLOCKED=3:
        user_role <= required && user_role != UserRole::BLOCKED as i32
    }
}

pub struct AdminOnly;
impl AccessLevel for AdminOnly {
    fn required_role() -> UserRole {
        UserRole::ADMIN
    }
}

pub struct AnyUser;
impl AccessLevel for AnyUser {
    fn required_role() -> UserRole {
        UserRole::USER
    }
}

impl<S> FromRequestParts<S> for AccessClaims
where
    AuthConfig: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_config = AuthConfig::from_ref(state);

        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let token_data = decode::<AccessClaims>(
            bearer.token(),
            &DecodingKey::from_secret(auth_config.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

pub struct RoleClaims<L: AccessLevel>(pub AccessClaims, PhantomData<L>);

impl<S, L> FromRequestParts<S> for RoleClaims<L>
where
    L: AccessLevel + Send + Sync,
    AuthConfig: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let access_claims = AccessClaims::from_request_parts(parts, state).await?;

        if L::is_satisfied(access_claims.role)
        {
            Ok(RoleClaims(access_claims, std::marker::PhantomData))
        } else {
            Err(AuthError::Forbidden)
        }
    }
}
