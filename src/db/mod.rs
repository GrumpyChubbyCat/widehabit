use diesel_async::{AsyncPgConnection, pooled_connection::bb8};
pub mod schema;

pub type PgPool = bb8::Pool<AsyncPgConnection>;