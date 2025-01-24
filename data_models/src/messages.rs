use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMessageRequest {
    pub channel_id: i32,
    pub content: String,
    pub attachments: Vec<i32>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageResponse {
    pub id: i32,
    pub author_id: i32,
    pub author: String,
    pub content: String,
    pub attachments: Vec<String>,
    pub created_date: chrono::DateTime<chrono::Utc>,
    pub updated_date: chrono::DateTime<chrono::Utc>,
}