use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Default, Debug)]
pub struct Channel {
    pub id: Option<i32>,
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date: Option<chrono::DateTime<chrono::Utc>>
}

#[derive(Serialize, Deserialize)]
pub struct CreateChannelRequest {
    pub name: String,
    pub guild_id: Option<i32>
}

impl From<CreateChannelRequest> for Channel {
    fn from(value: CreateChannelRequest) -> Self {
        Self {
            id: None,
            name: Some(value.name),
            message_count: None,
            guild_id: value.guild_id,
            created_date: None,
            updated_date: None,
        }
    }
}