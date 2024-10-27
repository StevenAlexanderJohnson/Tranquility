use serde::{Deserialize, Serialize};

use crate::{CreateChannelRequest, CreateGuildRequest, CreateMessageRequest};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum MessageData {
    Channel(CreateChannelRequest),
    Guild(CreateGuildRequest),
    Message(CreateMessageRequest),
    #[serde(rename = "ack", alias = "Ack")]
    Ack(String)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketMessage {
    pub data: MessageData,
}
