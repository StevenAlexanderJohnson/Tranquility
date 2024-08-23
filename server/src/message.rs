use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ReactionType {
    Like,
    Dislike,
    Love,
    Laugh,
    Sad,
    Angry,
    Wow,
    Care,
    Hug,
    Think,
}

impl ReactionType {
    fn as_str(&self) -> &str {
        match self {
            ReactionType::Like => "👍",
            ReactionType::Dislike => "👎",
            ReactionType::Love => "❤️",
            ReactionType::Laugh => "😂",
            ReactionType::Sad => "😢",
            ReactionType::Angry => "😡",
            ReactionType::Wow => "😮",
            ReactionType::Care => "😢",
            ReactionType::Hug => "🤗",
            ReactionType::Think => "🤔",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: ObjectId,
    pub text: String,
    pub author: ObjectId,
    pub reactions: HashMap<ReactionType, i32>,
    pub thread: Option<ObjectId>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageSet {
    pub offset: usize,
    pub count: usize,
    pub messages: Vec<Message>,
}