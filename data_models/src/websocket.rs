use serde::{Deserialize, Serialize};

use crate::{CreateChannelRequest, CreateGuildRequest};

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageData {
    Channel(CreateChannelRequest),
    Guild(CreateGuildRequest),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketMessage {
    pub message_type: String,
    pub data: Option<MessageData>,
}
