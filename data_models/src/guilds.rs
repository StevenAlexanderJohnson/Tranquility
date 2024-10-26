use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGuildRequest {
    pub name: String,
    pub description: String,
}
