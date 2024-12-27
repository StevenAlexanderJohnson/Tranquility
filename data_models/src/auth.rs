use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAuthUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthUserResponse {
    pub id: i32,
    pub username: String,
    pub token: String,
    pub refresh_token: String,
    pub websocket_token: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RefreshTokenRequest {
    pub refresh_token: String
}