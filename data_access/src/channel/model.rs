use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Default)]
pub struct Channel {
    pub id: Option<i32>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date: Option<chrono::DateTime<chrono::Utc>>
}