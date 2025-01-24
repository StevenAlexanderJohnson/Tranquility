use chrono::{DateTime, Utc};
use data_models::{CreateChannelRequest, CreateGuildRequest, CreateMessageRequest, MessageResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Notification {
    from: String,
    message: String,
    create_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum WebsocketMessageData {
    Channel(CreateChannelRequest),
    Guild(CreateGuildRequest),
    Message(CreateMessageRequest),
    Notification(Notification),
    #[serde(rename = "ack", alias = "Ack")]
    Ack(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WebsocketMessage {
    Data(WebsocketMessageData),
    #[serde(rename = "ping", alias = "Ping")]
    Ping(String)
}
// pub struct WebSocketMessage {
//     pub data: WebsocketMessageData,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum WebsocketResponseData {
    Message(MessageResponse),
    #[serde(rename = "ack", alias = "Ack")]
    Ack(String),
}