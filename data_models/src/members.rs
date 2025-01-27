use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMemberRequest {
    pub user_id: i32,
    pub guild_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMemberResponse {
    pub user_id: i32,
    pub guild_id: i32,
    pub created_date: chrono::DateTime<chrono::Utc>
}