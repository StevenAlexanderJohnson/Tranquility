use serde::{Deserialize, Serialize};
use sqlx::{
    types::chrono::{DateTime, Utc},
    FromRow,
};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Intent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub role_id: i32,
    pub value: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Role {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub name: String,
    pub guild_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date: Option<DateTime<Utc>>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct RoleResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub name: String,
    pub guild_id: i32,
    pub intents: Vec<Intent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date: Option<DateTime<Utc>>
}