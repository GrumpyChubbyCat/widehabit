use crate::db::schema::users;
use diesel::{HasQuery, Insertable};
use uuid::Uuid;

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

#[derive(Insertable, PartialEq, Debug)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
    pub role_id: i32,
}
