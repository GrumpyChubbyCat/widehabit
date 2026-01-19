use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Deserialize, Serialize)]
pub struct AuthToken {
    pub access_token: String,
}
