use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(ToSchema, Deserialize)]
pub struct UserAuthReq {
    pub username: String,
    pub password: String,
}

#[derive(ToSchema, Deserialize, Validate)]
pub struct UserRegistrationReq {
    #[validate(email(message = "invalid_email_format"))]
    pub email: String,
    #[validate(length(min = 3, max = 20, message = "invalid_username_length"))]
    pub username: String,
    #[validate(length(min = 6, message = "password_too_short"))]
    pub password: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum UserRole {
    ADMIN = 1,
    USER = 2,
    BLOCKED = 3,
}

impl From<i32> for UserRole {
    fn from(v: i32) -> Self {
        match v {
            1 => UserRole::ADMIN,
            2 => UserRole::USER,
            _ => UserRole::BLOCKED
        }
    }
}

pub struct UserRoleData {
    pub user_id: Uuid,
    pub role: UserRole,
}