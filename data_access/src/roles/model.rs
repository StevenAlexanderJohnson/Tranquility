use serde::{Deserialize, Serialize};
use sqlx::{
    types::chrono::{DateTime, Utc},
    FromRow,
};

#[derive(Debug, Serialize, Deserialize)]
#[repr(i32)]
pub enum IntentValue {
    GuildCreate = 100,
    GuildUpdate = 101,
    GuildDelete = 102,
    GuildRoleCreate = 103,
    GuildRoleUpdate = 104,
    GuildRoleDelete = 105,
    GuildMemberAdd = 106,
    GuildMemberUpdate = 107,
    GuildMemberRemove = 108,
    GuildBanAdd = 109,
    GuildBanRemove = 110,
    GuildEmojisUpdate = 111,
    GuildIntegrationsUpdate = 112,
    ChannelCreate = 200,
    ChannelUpdate = 201,
    ChannelDelete = 202,
    ChannelPinsUpdate = 203,
    MessageCreate = 300,
    MessageUpdate = 301,
    MessageDelete = 302,
    MessageDeleteBulk = 303,
    MessageReactionAdd = 304,
    MessageReactionRemove = 305,
    MessageReactionRemoveAll = 306,
    MessageReactionRemoveEmoji = 307,
}

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
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>
}