use diesel_async::{AsyncPgConnection, pooled_connection::bb8};
pub mod schema;
pub mod repo;
pub mod entity;

pub type DbPool = bb8::Pool<AsyncPgConnection>;