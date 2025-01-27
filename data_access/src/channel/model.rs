use data_models::CreateChannelResponse;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Default, Debug)]
pub struct Channel {
    pub id: Option<i32>,
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date: Option<chrono::DateTime<chrono::Utc>>,
}

impl TryFrom<Channel> for CreateChannelResponse {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: Channel) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value
                .id
                .ok_or("ID was not provided while casting from Channel to CreateChannelResponse")?,
            name: value
                .name
                .ok_or("Name was not provided while casting Channel to CreateCHannelResponse")?,
            guild_id: value.guild_id.ok_or(
                "Guild ID was not provided while casting Channel to CreateCHannelResponse",
            )?,
            created_date: value.created_date.ok_or(
                "Created Date was not provided while casting Channel to CreateCHannelResponse",
            )?,
            updated_date: value.updated_date.ok_or(
                "Updated Date was not provided while casting Channel to CreateCHannelResponse",
            )?,
        })
    }
}
