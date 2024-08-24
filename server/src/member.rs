use std::collections::HashSet;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::role::Role;

#[derive(Debug, Serialize, Deserialize)]
pub struct Member {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub auth_id: ObjectId,
    pub name: String,
    pub joined_date: chrono::DateTime<chrono::Utc>,
    pub friends: HashSet<ObjectId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerMember {
    pub member: ObjectId,
    pub roles: HashSet<Role>,
    pub joined_date: chrono::DateTime<chrono::Utc>,
}