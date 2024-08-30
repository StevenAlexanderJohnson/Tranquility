use sqlx::{Postgres, Transaction};

use crate::Channel;

#[derive(Clone)]
pub struct ChannelRepository {}

impl ChannelRepository {
    pub async fn insert(&self, channel: Channel, tx: &mut Transaction<'_, Postgres>) -> Result<Channel, Box<dyn std::error::Error>> {
        sqlx::query_as::<_, Channel>(
            "INSERT INTO channel (name, message_count) VALUES ($1, $2) RETURNS id, name, message_count;"
        )
        .bind(channel.name)
        .bind(channel.message_count)
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| e.into())
    }
}
