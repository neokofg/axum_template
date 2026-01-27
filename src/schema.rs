// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Char,
        email -> Varchar,
        password_hash -> Varchar,
        name -> Varchar,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
