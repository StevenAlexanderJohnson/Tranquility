use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateGuildRequest {
    pub name: String,
    pub description: String,
}
