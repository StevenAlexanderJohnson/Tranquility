use std::fmt;

use serde::{Deserialize, Serialize};
use sqlx::{types::chrono, FromRow};

use crate::Channel;

#[derive(Serialize, Deserialize, FromRow, Default, Debug)]
#[sqlx(default)]
pub struct Guild {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
    pub owner_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize)]
pub struct GuildResponse {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub owner_id: i32,
    pub channels: Vec<Channel>,
    pub created_date: chrono::DateTime<chrono::Utc>,
    pub updated_date: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct FromGuildError(String);
impl fmt::Display for FromGuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ERROR - Unable to convert Guild to GuildResult: {}",
            self.0
        )
    }
}
impl std::error::Error for FromGuildError {}

impl TryFrom<Guild> for GuildResponse {
    type Error = FromGuildError;

    fn try_from(value: Guild) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id.ok_or(FromGuildError(
                "ID was not provided while casting Guild to GuildResponse".into(),
            ))?,
            name: value.name,
            description: value.description,
            owner_id: value.owner_id.ok_or(FromGuildError(
                "Owner ID was not provided while casting Guild to GuildResponse".into(),
            ))?,
            channels: vec![],
            created_date: value.created_date.ok_or(FromGuildError(
                "Created Date was not provided while casting Guild to GuildResponse".into(),
            ))?,
            updated_date: value.updated_date.ok_or(FromGuildError(
                "Updated Date was not provided while casting Guild to GuildResponse".into(),
            ))?,
        })
    }
}
