use serde::{Deserialize, Serialize};
use sqlx::{
    FromRow,
    types::chrono::{DateTime, Utc}
};

#[derive(Serialize, Deserialize, FromRow, Default, Debug)]
#[sqlx(default)]
pub struct Message {
    pub id: Option<i32>,
    pub channel_id: Option<i32>,
    pub author_id: Option<i32>,
    pub content: Option<String>,
    pub created_date: Option<DateTime<Utc>>,
    pub updated_date: Option<DateTime<Utc>>
}

#[derive(Serialize, Deserialize)]
pub struct MessageResponse {
    pub id: i32,
    pub author_id: i32,
    pub content: String,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>
}