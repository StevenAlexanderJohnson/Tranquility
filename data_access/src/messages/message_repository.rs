use sqlx::{Postgres, Transaction};

use crate::Message;

use data_models::CreateMessageRequest;

#[derive(Clone)]
pub struct MessageRepository {}

impl MessageRepository {
    pub async fn insert(
        &self,
        message: &CreateMessageRequest,
        creator_id: i32,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        sqlx::query_as::<_, Message>(
            r"
        INSERT INTO message (author_id, channel_id, content)
        SELECT $1, $2, $3
        WHERE EXISTS (
            SELECT 1 FROM channel c
            join member m on c.guild_id = m.guild_id
            where m.user_id = $1 and c.id = $2
        )
        RETURNING id, channel_id, author_id, content, created_date, updated_date;
        ",
        )
        .bind(creator_id)
        .bind(message.channel_id)
        .bind(message.content.clone())
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| e.into())
    }
}
