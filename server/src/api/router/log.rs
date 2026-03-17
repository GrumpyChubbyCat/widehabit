use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use shared::model::{
    PagedResponse, PaginationParams,
    log::{HabitLogData, HabitStats, NewHabitLogReq},
};

use crate::{
    api::{
        extractors::{AnyUser, RoleClaims},
        state::AppState,
    },
    errors::InternalError,
    service::log::HabitLogService,
};

pub const LOG_TAG: &str = "log";

pub fn log_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(create_log))
        .routes(routes!(get_logs))
        .routes(routes!(get_habit_stats))
}

#[utoipa::path(
    get,
    path = "/{habit_id}/stats",
    tag = LOG_TAG,
    params(
        ("habit_id" = Uuid, Path, description = "Habit identifier"),
    ),
    responses(
        (status = OK, description = "Habit statistics", body = HabitStats)
    ),
    security(
        ("api_key" = [])
    )
)]
async fn get_habit_stats(
    State(log_service): State<Arc<HabitLogService>>,
    access_claims: RoleClaims<AnyUser>,
    Path(habit_id): Path<Uuid>,
) -> Result<Json<HabitStats>, InternalError> {
    let user_id = access_claims.0.sub;

    let stats = log_service.get_stats(habit_id, user_id).await?;

    Ok(Json(stats))
}

#[utoipa::path(
    get,
    path = "/{habit_id}",
    tag = LOG_TAG,
    params(
        ("habit_id" = Uuid, Path, description = "Habit identifier"),
        PaginationParams
    ),
    responses(
        (status = OK, description = "Habit logs list", body = inline(PagedResponse<HabitLogData>))
    ),
    security(
        ("api_key" = [])
    )
)]
async fn get_logs(
    State(log_service): State<Arc<HabitLogService>>,
    access_claims: RoleClaims<AnyUser>,
    Path(habit_id): Path<Uuid>,
    params: Query<PaginationParams>,
) -> Result<Json<PagedResponse<HabitLogData>>, InternalError> {
    let user_id = access_claims.0.sub;

    let paged_response = log_service
        .get_paged(habit_id, user_id, params.page, params.limit)
        .await?;

    Ok(Json(paged_response))
}

#[utoipa::path(
    post,
    path = "/{habit_id}",
    tag = LOG_TAG,
    params(
        ("habit_id" = Uuid, Path, description = "Habit identifier")
    ),
    request_body = NewHabitLogReq,
    responses(
        (status = CREATED, description = "Habit successfully logged", body = HabitLogData)
    ),
    security(
        ("api_key" = [])
    )
)]
async fn create_log(
    State(log_service): State<Arc<HabitLogService>>,
    access_claims: RoleClaims<AnyUser>,
    Path(habit_id): Path<Uuid>,
    Json(new_habit_log_req): Json<NewHabitLogReq>,
) -> Result<(StatusCode, Json<HabitLogData>), InternalError> {
    let user_id = access_claims.0.sub;

    let log_res = log_service
        .create(habit_id, user_id, new_habit_log_req)
        .await?;

    Ok((StatusCode::CREATED, Json(log_res)))
}
