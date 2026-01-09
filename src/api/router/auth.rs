use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use time::Duration as TimeDuration;

use crate::{
    api::{
        extractors::{AdminOnly, RoleClaims},
        router::AppState,
    },
    errors::InternalError,
    model::{auth::AuthToken, user::UserAuthReq},
    service::user::UserService,
};

const COOKIE_LIFETIME: i64 = 30;

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(auth_user))
        .route("/admin", get(admin_route))
}

pub async fn auth_user(
    State(user_serivce): State<Arc<UserService>>,
    jar: CookieJar,
    Json(user_info): Json<UserAuthReq>,
) -> Result<(CookieJar, Json<AuthToken>), InternalError> {
    let user_data = user_serivce.authenticate(user_info).await?;

    let access_token = user_serivce
        .create_access_token(user_data.user_id, user_data.role as i32)
        .await?;
    let refresh_token = user_serivce.create_refresh_token(user_data.user_id).await?;

    let cookie = Cookie::build(("refresh_token", refresh_token.clone()))
        .path("/")
        .http_only(true) // Closed access for JS
        .same_site(SameSite::Strict) // CSRF protection
        .max_age(TimeDuration::days(COOKIE_LIFETIME))
        .build();

    Ok((jar.add(cookie), Json(AuthToken { access_token })))
}

pub async fn admin_route(_access_claims: RoleClaims<AdminOnly>) -> &'static str {
    "Welcome to admin route!"
}
