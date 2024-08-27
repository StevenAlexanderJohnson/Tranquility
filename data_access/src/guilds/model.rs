use serde::{Deserialize, Serialize};
use sqlx::{types::chrono, FromRow};

#[derive(Serialize, Deserialize, FromRow, Default)]
#[sqlx(default)]
pub struct Guild {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
    pub owner_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date: Option<chrono::DateTime<chrono::Utc>>
}
