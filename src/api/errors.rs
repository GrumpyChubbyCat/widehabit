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
            InternalError::AlreadyExists => (StatusCode::CONFLICT, self.to_string()),
            InternalError::Blocked => (StatusCode::FORBIDDEN, self.to_string()),
            InternalError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            InternalError::Validation(error) => (StatusCode::UNPROCESSABLE_ENTITY, error.to_string()),
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

#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
    Forbidden,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token!"),
            AuthError::Forbidden => (StatusCode::FORBIDDEN, "Access denied!"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
