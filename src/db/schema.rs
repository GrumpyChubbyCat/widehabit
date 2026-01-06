// @generated automatically by Diesel CLI.

diesel::table! {
    roles (role_id) {
        role_id -> Int4,
        title -> Varchar,
        about -> Text,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        refresh_hash -> Nullable<Varchar>,
        role_id -> Int4,
    }
}

diesel::joinable!(users -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(roles, users,);
