use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
    pub guild_id: i32,
    pub intents: Vec<IntentValue>,
}
