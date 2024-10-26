use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateAuthUserRequest {
    pub username: String,
    pub password: String,
    pub email: String
}
