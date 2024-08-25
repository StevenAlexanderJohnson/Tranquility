use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

#[derive(Serialize, Deserialize, FromRow, Default)]
pub struct AuthUser {
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub refresh_token: String,
    pub claims: Option<Vec<String>>,
}
