use serde::{Deserialize, Serialize};
use sqlx::{
    self,
    types::chrono::{DateTime, Utc},
    FromRow,
};

#[derive(Serialize, Deserialize, FromRow, Default, Clone, Debug)]
#[sqlx(default)]
pub struct AuthUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateAuthUserRequest {
    pub username: String,
    pub password: String,
    pub email: String
}

impl From<CreateAuthUserRequest> for AuthUser {
    fn from(value: CreateAuthUserRequest) -> Self {
        AuthUser {
            id: None,
            username: value.username,
            password: Some(value.password),
            email: Some(value.email),
            refresh_token: None,
            created_date: None,
            updated_date: None
        }
    }
}