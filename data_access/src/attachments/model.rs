use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow, Default, Debug)]
pub struct Attachment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub file_name: Option<String>,
    pub file_path: Option<String>,
    pub file_size: Option<i64>,
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<chrono::DateTime<chrono::Utc>>,
}

pub struct AttachmentResponse {
    pub id: i32,
    pub name: String,
    pub file_name: String,
    pub file_path: String,
    pub mime_type: String,
    pub created_date: chrono::DateTime<chrono::Utc>
}
