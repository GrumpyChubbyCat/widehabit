use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct UserAuthReq {
    pub username: String,
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