use std::time::Duration;

use axum::{Router, body::Body, extract::Request, response::Response, routing::get};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::Span;

use crate::db::PgPool;

pub fn api_router(db: PgPool) -> Router {
    let trace_layer = TraceLayer::new_for_http()
        .on_request(|request: &Request<Body>, _span: &Span| {
            tracing::info!(
                "Started processing request method={} path={} version={:?}",
                request.method(),
                request.uri().path(),
                request.version()
            )
        })
        .on_response(|response: &Response, latency: Duration, _span: &Span| {
            tracing::info!(
                "Response status={} in {:?} secs",
                response.status(),
                latency.as_secs_f32()
            )
        })
        .on_failure(
            |error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                tracing::error!("Error {}", error)
            },
        );

    Router::new()
        .route("/", get(root_get))
        .layer(trace_layer)
        .with_state(db)
}

async fn root_get() -> &'static str {
    "Hi from widehabit"
}
