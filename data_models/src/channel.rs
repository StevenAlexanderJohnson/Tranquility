use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateChannelRequest {
    pub name: String,
    pub guild_id: Option<i32>
}
