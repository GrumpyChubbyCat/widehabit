use diesel::HasQuery;
use uuid::Uuid;
use crate::db::schema::users;

#[derive(HasQuery, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub refresh_hash: Option<String>,
    pub role_id: i32,
}