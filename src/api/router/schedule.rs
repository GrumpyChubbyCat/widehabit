use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;
use validator::Validate;

use crate::{
    api::{
        extractors::{AnyUser, RoleClaims},
        state::AppState,
    },
    errors::InternalError,
    model::schedule::{ScheduleRes, SetScheduleReq},
    service::schedule::HabitScheduleService,
};

pub const SCHEDULE_TAG: &str = "schedule";

pub fn schedule_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(set_schedule))
        .routes(routes!(get_schedule))
}

#[utoipa::path(
    get, 
    path = "/{habit_id}",
    tag = SCHEDULE_TAG,
    params(
        ("habit_id" = Uuid, Path, description = "Habit identifier")
    ),
    responses(
        (status = OK, description = "Habit schedule find successfully", body = ScheduleRes),
        (status = NOT_FOUND, description = "Habit schedule not found")
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn get_schedule(
    State(schedule_service): State<Arc<HabitScheduleService>>,
    access_claims: RoleClaims<AnyUser>,
    Path(habit_id): Path<Uuid>,
) -> Result<Json<ScheduleRes>, InternalError> {
    let user_id = access_claims.0.sub;

    let schedules_res = schedule_service.get(habit_id, user_id).await?;

    Ok(Json(ScheduleRes {
        schedules: schedules_res,
    }))
}

#[utoipa::path(
    put,
    path = "/",
    tag = SCHEDULE_TAG,
    request_body = SetScheduleReq,
    responses(
        (status = OK, description = "New habit schedule has been created", body = ScheduleRes)
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn set_schedule(
    State(schedule_service): State<Arc<HabitScheduleService>>,
    access_claims: RoleClaims<AnyUser>,
    Json(set_shcedule_req): Json<SetScheduleReq>,
) -> Result<Json<ScheduleRes>, InternalError> {

    set_shcedule_req.validate().map_err(|e| InternalError::Validation(e.to_string()))?;

    let user_id = access_claims.0.sub;

    let schedules_res = schedule_service
        .update(
            set_shcedule_req.habit_id,
            user_id,
            set_shcedule_req.schedules,
        )
        .await?;

    Ok(Json(ScheduleRes {
        schedules: schedules_res,
    }))
}
