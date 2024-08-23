use std::collections::HashMap;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Message {
    pub id: uuid::Uuid,
    pub text: String,
    pub author: uuid::Uuid,
    pub reactions: HashMap<ReactionType, i32>,
    pub thread: Option<uuid::Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct MessageSet {
    pub offset: usize,
    pub count: usize,
    pub messages: Vec<Message>,
}