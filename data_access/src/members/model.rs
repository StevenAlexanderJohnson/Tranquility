use serde::{Deserialize, Serialize};
use sqlx::{
    types::chrono::{DateTime, Utc},
    FromRow,
};

#[derive(Serialize, Deserialize, FromRow, Default, Debug)]
#[sqlx(default)]
pub struct Member {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub guild_id: Option<i32>,
    pub user_who_added: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date: Option<DateTime<Utc>>,
}
