use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AttachmentResponse {
    pub id: i32,
    pub file_name: String,
    pub file_size: i64,
    pub mime_type: String,
    pub created_date: chrono::DateTime<chrono::Utc>,
}
