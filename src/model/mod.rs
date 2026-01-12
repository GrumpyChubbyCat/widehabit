use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

pub mod auth;
pub mod habit;
pub mod user;

#[derive(ToSchema, Serialize)]
pub struct PagedResponse<T> {
    pub items: Vec<T>,
    pub total_count: i64,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Deserialize, IntoParams)]
pub struct PaginationParams {
    pub page: i64,
    pub limit: i64,
}