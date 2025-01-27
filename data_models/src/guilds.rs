use serde::{Deserialize, Serialize};

use crate::CreateChannelResponse;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGuildRequest {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateGuildResponse {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub owner_id: i32,
    pub channels: Vec<CreateChannelResponse>,
    pub created_date: chrono::DateTime<chrono::Utc>,
    pub updated_date: chrono::DateTime<chrono::Utc>,
}
