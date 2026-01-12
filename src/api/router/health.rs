use axum::http::StatusCode;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::api::state::AppState;

pub const HEALTH_TAG: &str = "health";

pub fn health_router() ->OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(healthcheck))
}

#[utoipa::path(
    method(get),
    path = "/healthcheck",
    tag = HEALTH_TAG,
    responses(
        (status = OK, description = "Success")
    )
)]
async fn healthcheck() -> StatusCode {
    StatusCode::OK
}