pub mod auth;
pub mod habit;
pub mod health;
pub mod schedule;

use std::time::Duration;

use axum::{Router, body::Body, extract::Request, response::Response};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::Span;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

#[cfg(debug_assertions)]
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    api::{
        docs::WideApiDoc,
        router::{auth::auth_router, habit::habit_router, health::health_router, schedule::schedule_router},
        state::AppState,
    },
    config::AuthConfig,
    db::{
        DbPool,
        repo::{habit::HabitRepository, schedule::HabitScheduleRepository, user::UserRepository},
    },
    service::{habit::HabitService, schedule::HabitScheduleService, user::UserService},
};

const API_PREFIX: &str = "/api/v1";

pub fn api_router(db_pool: DbPool, auth_config: AuthConfig) -> Router {

    let trace_layer = TraceLayer::new_for_http()
        .on_request(|_request: &Request<Body>, _span: &Span| tracing::info!("request_started",))
        .on_response(|response: &Response, latency: Duration, _span: &Span| {
            let res_status = response.status();
            let latency_ms = latency.as_millis();
            let status = res_status.as_u16();

            if res_status.is_server_error() {
                tracing::error!(status, latency_ms, "server_error");
            } else if res_status.is_client_error() {
                tracing::warn!(status, latency_ms, "client_error");
            } else {
                tracing::info!(status, latency_ms, "request_success")
            }
        })
        .on_failure(
            |error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                tracing::error!("app_error {}", error)
            },
        );

    let user_repo = UserRepository::new(db_pool.clone());
    let habit_repo = HabitRepository::new(db_pool.clone());
    let schedule_repo = HabitScheduleRepository::new(db_pool.clone());

    let user_service = UserService::new(user_repo, auth_config.clone());
    let habit_service = HabitService::new(habit_repo);
    let schedule_service = HabitScheduleService::new(schedule_repo);

    let app_state = AppState::new(auth_config, user_service, habit_service, schedule_service);

    let api_routes = OpenApiRouter::new()
        .nest("/health", health_router())
        .nest("/auth", auth_router())
        .nest("/habit", habit_router())
        .nest("/shcedule", schedule_router());

    let (router, _api) = OpenApiRouter::with_openapi(WideApiDoc::openapi())
        .nest(API_PREFIX, api_routes)
        .layer(trace_layer)
        .with_state(app_state)
        .split_for_parts();

    #[cfg(debug_assertions)]
    let router = router.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", _api));

    router
}
