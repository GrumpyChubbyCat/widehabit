use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
    api::{
        extractors::{AnyUser, RoleClaims},
        state::AppState,
    },
    errors::InternalError,
    model::{
        PagedResponse, PaginationParams,
        habit::{HabitData, NewHabitReq},
    },
    service::habit::HabitService,
};

pub const HABIT_TAG: &str = "habit";

pub fn habit_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(create_habit))
        .routes(routes!(get_habit))
        .routes(routes!(get_habits))
}

#[utoipa::path(
    post, 
    path = "/", 
    tag = HABIT_TAG, 
    request_body = NewHabitReq, 
    responses(
        (status = CREATED, description = "New habit has been created")
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn create_habit(
    State(habit_service): State<Arc<HabitService>>,
    access_claims: RoleClaims<AnyUser>,
    Json(new_habit_req): Json<NewHabitReq>,
) -> Result<StatusCode, InternalError> {
    let user_id = access_claims.0.sub;

    habit_service.add_new(new_habit_req, user_id).await?;

    Ok(StatusCode::CREATED)
}

#[utoipa::path(
    get, 
    path = "/{habit_id}",
    tag = HABIT_TAG,
    params(
        ("habit_id" = Uuid, Path, description = "Habit identifier")
    ),
    responses(
        (status = 200, description = "Single habit", body = HabitData),
        (status = 404, description = "Habit not found or access denied")
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn get_habit(
    State(habit_service): State<Arc<HabitService>>,
    access_claims: RoleClaims<AnyUser>,
    Path(habit_id): Path<Uuid>
) -> Result<Json<HabitData>, InternalError> {
    let user_id = access_claims.0.sub;

    let habit_data = habit_service
        .get_by_id(habit_id, user_id).await?;

    Ok(Json(habit_data))
}

#[utoipa::path(
    get, 
    path = "/", 
    tag = HABIT_TAG, 
    params(PaginationParams),
    responses(
        (status = 200, description = "Habits list", body = inline(PagedResponse<HabitData>))
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn get_habits(
    State(habit_service): State<Arc<HabitService>>,
    access_claims: RoleClaims<AnyUser>,
    params: Query<PaginationParams>,
) -> Result<Json<PagedResponse<HabitData>>, InternalError> {
    let user_id = access_claims.0.sub;

    let paged_response = habit_service
        .get_paged(user_id, params.page, params.limit)
        .await?;

    Ok(Json(paged_response))
}
