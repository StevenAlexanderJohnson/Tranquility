use serde::{Deserialize, Serialize};
use sqlx::{types::chrono, FromRow};

#[derive(Serialize, Deserialize, FromRow, Default)]
pub struct Guild {
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
    pub created_date: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_date: Option<chrono::DateTime<chrono::Utc>>
}
