use diesel::Queryable;
use chrono::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub screen_name: String,
    pub display_name: String,
    pub password_hash: String,
    pub remember_token: Option<String>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}
