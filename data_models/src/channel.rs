use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateChannelRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateChannelResponse {
    pub id: i32,
    pub name: String,
    pub guild_id: i32,
    pub created_date: chrono::DateTime<chrono::Utc>,
    pub updated_date: chrono::DateTime<chrono::Utc>,
}