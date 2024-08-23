use std::collections::HashSet;
use crate::message::MessageSet;

pub struct Channel {
    pub id: uuid::Uuid,
    pub name: String,
    pub message_count: u128,
    pub message_set: MessageSet,
    pub allowed_roles: HashSet<uuid::Uuid>,
}