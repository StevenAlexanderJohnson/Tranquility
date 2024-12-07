use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAttachmentResponse {
    pub id: i32,
    pub name: String,
    pub file_name: String,
    pub file_path: String,
    pub file_size: f32,
    pub mime_type: String,
}