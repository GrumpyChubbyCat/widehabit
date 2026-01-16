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
        habit::{HabitData, NewHabitReq, UpdateHabitRes},
    },
    service::habit::HabitService,
};

pub const HABIT_TAG: &str = "habit";

pub fn habit_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(create_habit))
        .routes(routes!(update_habit))
        .routes(routes!(get_habit))
        .routes(routes!(get_habits))
        .routes(routes!(delete_habit))
}

#[utoipa::path(
    post, 
    path = "/", 
    tag = HABIT_TAG, 
    request_body = NewHabitReq, 
    responses(
        (status = CREATED, description = "New habit has been created", body = HabitData)
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn create_habit(
    State(habit_service): State<Arc<HabitService>>,
    access_claims: RoleClaims<AnyUser>,
    Json(new_habit_req): Json<NewHabitReq>,
) -> Result<(StatusCode, Json<HabitData>), InternalError> {
    let user_id = access_claims.0.sub;

    let added_habit = habit_service.create(new_habit_req, user_id).await?;

    Ok((StatusCode::CREATED, Json(added_habit)))
}

#[utoipa::path(
    get, 
    path = "/{habit_id}",
    tag = HABIT_TAG,
    params(
        ("habit_id" = Uuid, Path, description = "Habit identifier")
    ),
    responses(
        (status = OK, description = "Single habit", body = HabitData),
        (status = NOT_FOUND, description = "Habit not found or access denied")
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
        (status = OK, description = "Habits list", body = inline(PagedResponse<HabitData>))
    ),
    security(
        ("api_key" = [])
    )
)]
async fn get_habits(
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

#[utoipa::path(
    patch, 
    path = "/{habit_id}",
    tag = HABIT_TAG, 
    responses(
        (status = OK, description = "Update existent habit", body = UpdateHabitRes)
    ),
    security(
        ("api_key" = [])
    )
)]
async fn update_habit(
    State(habit_service): State<Arc<HabitService>>,
    access_claims: RoleClaims<AnyUser>,
    Path(habit_id): Path<Uuid>,
    Json(new_habit): Json<NewHabitReq>,
) -> Result<Json<UpdateHabitRes>, InternalError> {
    let user_id = access_claims.0.sub;

    habit_service.update(habit_id, user_id, &new_habit).await?;

    Ok(Json(
        UpdateHabitRes { habit_id, name: new_habit.name, description: new_habit.description }
    ))
}

#[utoipa::path(
    delete, 
    path = "/{habit_id}", 
    tag = HABIT_TAG, 
    responses(
        (status = NO_CONTENT, description = "Delete existent habit")
    ),
    security(
        ("api_key" = [])
    )
)]
async fn delete_habit(
    State(habit_service): State<Arc<HabitService>>,
    access_claims: RoleClaims<AnyUser>,
    Path(habit_id): Path<Uuid>,
) -> Result<StatusCode, InternalError> {
    let user_id = access_claims.0.sub;
    
    habit_service.delete(habit_id, user_id).await?;

    Ok(StatusCode::NO_CONTENT)
}