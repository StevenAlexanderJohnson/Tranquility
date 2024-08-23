use std::collections::HashSet;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::role::Role;

#[derive(Debug, Serialize, Deserialize)]
pub struct Member {
    pub id: ObjectId,
    pub name: String,
    pub joined_date: chrono::DateTime<chrono::Utc>,
    pub friends: HashSet<ObjectId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerMember {
    pub member: Member,
    pub roles: HashSet<Role>,
    pub joined_date: chrono::DateTime<chrono::Utc>,
}