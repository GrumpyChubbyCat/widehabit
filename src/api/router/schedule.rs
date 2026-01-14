use std::sync::Arc;

use axum::{Json, extract::State};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    api::{
        extractors::{AnyUser, RoleClaims},
        state::AppState,
    },
    errors::InternalError,
    model::schedule::{SetScheduleReq, SetScheduleRes},
    service::schedule::HabitScheduleService,
};

pub const SCHEDULE_TAG: &str = "schedule";

pub fn schedule_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(set_schedule))
}

#[utoipa::path(
    put,
    path = "/",
    tag = SCHEDULE_TAG,
    request_body = SetScheduleReq,
    responses(
        (status = OK, description = "New habit schedule has been created", body = SetScheduleRes)
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn set_schedule(
    State(schedule_service): State<Arc<HabitScheduleService>>,
    access_claims: RoleClaims<AnyUser>,
    Json(set_shcedule_req): Json<SetScheduleReq>,
) -> Result<Json<SetScheduleRes>, InternalError> {
    let user_id = access_claims.0.sub;

    let schedules_res = schedule_service
        .update_schedule(
            set_shcedule_req.habit_id,
            user_id,
            set_shcedule_req.schedules,
        )
        .await?;

    Ok(Json(SetScheduleRes {
        schedules: schedules_res,
    }))
}
