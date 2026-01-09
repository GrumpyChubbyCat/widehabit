use crate::errors::InternalError;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

impl IntoResponse for InternalError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            InternalError::InvalidCredentials => (StatusCode::UNAUTHORIZED, self.to_string()),
            InternalError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            _ => {
                tracing::error!("Internal server error: {:?}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
