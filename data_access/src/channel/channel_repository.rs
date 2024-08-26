use sqlx::{Pool, Postgres};

use crate::Channel;

pub struct ChannelRepository {
    pool: Pool<Postgres>,
}

impl ChannelRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, channel: Channel) -> Result<Channel, Box<dyn std::error::Error>> {
        sqlx::query_as::<_, Channel>(
            "INSERT INTO channel (name, message_count) VALUES ($1, $2) RETURNS id, name, message_count;"
        )
        .bind(channel.name)
        .bind(channel.message_count)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.into())
    }
}
