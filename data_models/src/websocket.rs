use serde::{Deserialize, Serialize};

use crate::{CreateChannelRequest, CreateGuildRequest, CreateMessageRequest};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum MessageData {
    Hello(Hello),
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Hello {
    pub token: String,
    pub id: i32
}