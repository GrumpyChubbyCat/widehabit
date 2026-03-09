#[derive(Debug, Clone)]
pub enum ApiError {
    /// Network connectivity issues or server downtime
    NetworkError,
    /// HTTP 400 Bad Request or validation failures
    BadRequest(String),
    /// HTTP 401 Unauthorized - access required and refresh token failed
    AuthRequired,
    /// HTTP 403 Forbidden - insufficient permissions
    Forbidden,
    /// HTTP 500+ Internal Server Error
    ServerError,
    /// Failure during JSON/CBOR response parsing
    DecodeError,
}