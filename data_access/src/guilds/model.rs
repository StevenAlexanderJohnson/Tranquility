use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Default)]
pub struct Guild {
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
}
