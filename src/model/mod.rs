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

#[derive(Deserialize, IntoParams)] // IntoParams нужен для генерации Swagger
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_page() -> i64 {
    1
}
fn default_limit() -> i64 {
    20
}
