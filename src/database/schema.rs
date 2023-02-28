// @generated automatically by Diesel CLI.

diesel::table! {
    profiles (id) {
        id -> Int4,
        user_id -> Int4,
        profile_token -> Varchar,
        name -> Varchar,
        surname -> Varchar,
        email -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        depends_on -> Int4,
        role_id -> Int4,
        user_token -> Nullable<Varchar>,
        fcm_token -> Nullable<Varchar>,
        active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(users -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(profiles, roles, users,);
