use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateMemberRequest {
    pub user_id: i32,
    pub guild_id: i32,
}