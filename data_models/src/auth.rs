use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAuthUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String
}
