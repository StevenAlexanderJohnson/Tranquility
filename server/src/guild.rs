use crate::{channel::Channel, member::Member, role::Role};
use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Guild {
    pub id: bson::oid::ObjectId,
    pub name: String,
    pub description: String,
    pub channels: Vec<Channel>,
    pub members: Vec<Member>,
    pub roles: Vec<Role>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Guild {
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}