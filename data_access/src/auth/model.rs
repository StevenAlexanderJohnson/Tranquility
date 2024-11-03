use data_models::{AuthUserResponse, CreateAuthUserRequest};
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

impl From<CreateAuthUserRequest> for AuthUser {
    fn from(value: CreateAuthUserRequest) -> Self {
        AuthUser {
            id: None,
            username: value.username,
            password: Some(value.password),
            email: Some(value.email),
            refresh_token: None,
            created_date: None,
            updated_date: None,
        }
    }
}

impl TryFrom<AuthUser> for AuthUserResponse {
    type Error = &'static str;

    fn try_from(value: AuthUser) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id.ok_or("Id was not returned from the database")?,
            username: value.username,
            token: String::from(""),
            refresh_token: value.refresh_token.ok_or("Refresh token was not provided from the database")?,
        })
    }
}