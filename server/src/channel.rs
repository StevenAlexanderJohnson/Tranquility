use std::collections::HashSet;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::message::MessageSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub id: ObjectId,
    pub name: String,
    pub message_count: u128,
    pub message_set: MessageSet,
    pub allowed_roles: HashSet<ObjectId>,
}