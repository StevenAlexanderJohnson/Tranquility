use data_models::AttachmentResponse;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow, Default, Debug)]
pub struct Attachment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    pub file_size: Option<i64>,
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, FromRow, Default, Debug)]
pub struct AttachmentMapping {
    pub post_id: i32,
    pub attachment_id: i32,
}

impl TryFrom<&Attachment> for AttachmentResponse {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &Attachment) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id.ok_or("ID was not provided while casting from Attachment to AttachmentResponse")?,
            file_name: value.file_name.clone().ok_or("File Name was not provided while casting from Attachment to Attachment Response")?,
            file_size: value.file_size.ok_or("File Size was not provided while casting from Attachment to Attachment Response")?,
            mime_type: value.mime_type.clone().ok_or("Mime Type was not provided while casting from Attachment to Attachment Response")?,
            created_date: value.created_date.ok_or("Created Date was not provided while casting from Attachment to Attachment Response")?,
        })
    }
}
