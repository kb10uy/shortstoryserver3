diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        screen_name -> Varchar,
        display_name -> Varchar,
        password_hash -> Varchar,
        remember_token -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
