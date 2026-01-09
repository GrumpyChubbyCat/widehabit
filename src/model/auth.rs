use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessClaims {
    pub sub: Uuid, // user_id
    pub role: i32, // role_id
    pub exp: i64,  // expiration timestamp
    pub iat: i64,  // issued at
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: Uuid, // user_id
    pub exp: i64,  // expiration timestamp
    pub iat: i64,  // issued at
    pub jti: Uuid, // Unique Token ID
}

#[derive(Deserialize, Serialize)]
pub struct AuthToken {
    pub access_token: String,
}
