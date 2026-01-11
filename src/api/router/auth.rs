use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use time::Duration as TimeDuration;
use utoipa_axum::{router::OpenApiRouter, routes};
use validator::Validate;

use crate::{
    api::{
        extractors::{AnyUser, RoleClaims},
        router::AppState,
    },
    errors::InternalError,
    model::{
        auth::{AuthToken, RefreshClaims},
        user::{UserAuthReq, UserRegistrationReq, UserRole},
    },
    service::user::UserService,
};

const COOKIE_LIFETIME: i64 = 30;
pub const AUTH_TAG: &str = "auth";

pub fn auth_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(register_user))
        .routes(routes!(auth_user))
        .routes(routes!(refresh_access_token))
        .routes(routes!(logout))
}

#[utoipa::path(post, path = "/registration", tag = AUTH_TAG, request_body = UserRegistrationReq, responses((status = CREATED)))]
pub async fn register_user(
    State(user_service): State<Arc<UserService>>,
    Json(user_register_req): Json<UserRegistrationReq>,
) -> Result<StatusCode, InternalError> {
    user_register_req
        .validate()
        .map_err(|_| InternalError::Validation)?;

    user_service.register_user(user_register_req).await?;

    Ok(StatusCode::CREATED)
}

#[utoipa::path(post, path = "/login", tag = AUTH_TAG, request_body = UserAuthReq, responses((status = OK, body=AuthToken)))]
pub async fn auth_user(
    State(user_service): State<Arc<UserService>>,
    jar: CookieJar,
    Json(user_info): Json<UserAuthReq>,
) -> Result<(CookieJar, Json<AuthToken>), InternalError> {
    let user_data = user_service.authenticate(user_info).await?;

    let access_token = user_service
        .create_access_token(user_data.user_id, user_data.role as i32)
        .await?;
    let refresh_token = user_service.create_refresh_token(user_data.user_id).await?;

    let cookie = Cookie::build(("refresh_token", refresh_token.clone()))
        .path("/")
        .http_only(true) // Closed access for JS
        .same_site(SameSite::Strict) // CSRF protection
        .max_age(TimeDuration::days(COOKIE_LIFETIME))
        .build();

    Ok((jar.add(cookie), Json(AuthToken { access_token })))
}

#[utoipa::path(post, path = "/refresh", tag = AUTH_TAG, responses((status = OK, body=AuthToken)))]
pub async fn refresh_access_token(
    State(user_service): State<Arc<UserService>>,
    refresh_claims: RefreshClaims,
) -> Result<Json<AuthToken>, InternalError> {
    let user_data = user_service.get_user_by_id(refresh_claims.sub).await?;

    if user_data.role == UserRole::BLOCKED {
        return Err(InternalError::Blocked);
    }

    let access_token = user_service
        .create_access_token(user_data.user_id, user_data.role as i32)
        .await?;

    Ok(Json(AuthToken { access_token }))
}

#[utoipa::path(post, path = "/logout", tag = AUTH_TAG, responses((status = NO_CONTENT)), security(("api_key" = [])))]
pub async fn logout(
    State(user_service): State<Arc<UserService>>,
    jar: CookieJar,
    access_claims: RoleClaims<AnyUser>,
) -> Result<(CookieJar, StatusCode), InternalError> {
    user_service
        .delete_refresh_token(access_claims.0.sub)
        .await?;

    let cookie_to_remove = Cookie::build(("refresh_token", ""))
        .path("/")
        .max_age(TimeDuration::days(COOKIE_LIFETIME))
        .build();

    let updated_jar = jar.add(cookie_to_remove);

    Ok((updated_jar, StatusCode::NO_CONTENT))
}
