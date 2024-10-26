use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMessageRequest {
    pub channel_id: i32,
    pub content: String,
}