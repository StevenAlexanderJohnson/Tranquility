use sqlx::{Pool, Postgres, Transaction};

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
        WITH im AS (
        INSERT INTO message (author_id, channel_id, content)
        SELECT $1, $2, $3
        WHERE EXISTS (
            SELECT 1 FROM channel c
            join member m on c.guild_id = m.guild_id
            where m.user_id = $1 and c.id = $2
        )
        RETURNING id, channel_id, author_id, content, created_date, updated_date
        )
        SELECT
            im.id,
            im.channel_id,
            a.username as author,
            im.author_id,
            im.content,
            im.created_date,
            im.updated_date
        FROM im
        JOIN auth a ON im.author_id = a.id
        ",
        )
        .bind(creator_id)
        .bind(message.channel_id)
        .bind(message.content.clone())
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| e.into())
    }

    pub async fn get_page(
        &self,
        offset: i32,
        page_number: i32,
        user_id: i32,
        guild_id: i32,
        channel_id: i32,
        pool: &Pool<Postgres>,
    ) -> Result<Option<Vec<Message>>, Box<dyn std::error::Error>> {
        match sqlx::query_as::<_, Message>(
            r"
            SELECT
                m.id,
                m.channel_id,
                a.username as author,
                m.author_id,
                m.content,
                m.created_date,
                m.updated_date
            FROM message m
            JOIN auth a on a.id = m.author_id
            JOIN channel c ON m.channel_id = c.id
            JOIN guild g ON c.guild_id = g.id
            JOIN member mem ON mem.guild_id = g.id
            WHERE   g.id = $1
                AND c.id = $2
                AND mem.user_id = $3
            OFFSET $4 * $5
            LIMIT $4;
            ",
        )
        .bind(guild_id)
        .bind(channel_id)
        .bind(user_id)
        .bind(offset)
        .bind(page_number)
        .fetch_all(pool)
        .await
        {
            Ok(x) => Ok(Some(x)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
