use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Default)]
pub struct Channel {
    pub id: Option<i32>,
    pub name: String,
    pub message_count: i32,
}