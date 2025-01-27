use data_models::CreateMemberResponse;
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

impl TryFrom<Member> for CreateMemberResponse {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: Member) -> Result<Self, Self::Error> {
        Ok(Self {
            user_id: value.user_id.ok_or(
                "User ID was not provided while casting from Member to CreateMemberResponse",
            )?,
            guild_id: value.guild_id.ok_or(
                "Guild ID was not provided while casting from Member to CreateMemberResponse",
            )?,
            created_date: value.created_date.ok_or(
                "Created Date was not provided while casting from Member to CreateMemberResponse",
            )?,
        })
    }
}
