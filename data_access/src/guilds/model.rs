use std::fmt;

use data_models::CreateGuildResponse;
use serde::{Deserialize, Serialize};
use sqlx::{types::chrono, FromRow};

#[derive(Serialize, Deserialize, FromRow, Default, Debug)]
#[sqlx(default)]
pub struct Guild {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub owner_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date: Option<chrono::DateTime<chrono::Utc>>,
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

impl TryFrom<Guild> for CreateGuildResponse {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: Guild) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value
                .id
                .ok_or("ID was not provided while casting Guild to GuildResponse")?,
            name: value
                .name
                .ok_or("Guild name was not provided while casting Guild to GuildResponse")?,
            description: value
                .description
                .ok_or("Description was not provided while casting Guild to GuildResponse")?,
            owner_id: value
                .owner_id
                .ok_or("Owner ID was not provided while casting Guild to GuildResponse")?,
            channels: vec![],
            created_date: value
                .created_date
                .ok_or("Created Date was not provided while casting Guild to GuildResponse")?,
            updated_date: value
                .updated_date
                .ok_or("Updated Date was not provided while casting Guild to GuildResponse")?,
        })
    }
}
