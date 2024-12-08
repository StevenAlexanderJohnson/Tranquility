use serde::{Deserialize, Serialize};

use crate::{CreateChannelRequest, CreateGuildRequest, CreateMessageRequest, MessageResponse};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum WebsocketMessageData {
    Channel(CreateChannelRequest),
    Guild(CreateGuildRequest),
    Message(CreateMessageRequest),
    #[serde(rename = "ack", alias = "Ack")]
    Ack(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketMessage {
    pub data: WebsocketMessageData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum WebsocketResponseData {
    Message(MessageResponse),
    #[serde(rename = "ack", alias = "Ack")]
    Ack(String),
}
