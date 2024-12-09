use sqlx::{Pool, Postgres, Transaction};

use crate::Channel;

use data_models::CreateChannelRequest;

#[derive(Clone)]
pub struct ChannelRepository {}

impl ChannelRepository {
    pub async fn insert(
        &self,
        channel: &CreateChannelRequest,
        guild_id: i32,
        user_id: i32,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<Channel>, Box<dyn std::error::Error>> {
        match sqlx::query_as::<_, Channel>(
            r#"
            INSERT INTO channel (name, guild_id) SELECT $1, $2 WHERE EXISTS (SELECT 1 FROM member WHERE guild_id = $2 AND user_id = $3)
            RETURNING id, name, message_count, guild_id, created_date, updated_date;
            "#,
        )
        .bind(&channel.name)
        .bind(guild_id)
        .bind(&user_id)
        .fetch_one(&mut **tx)
        .await
        {
            Ok(channel) => Ok(Some(channel)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn find_guild_channels(
        &self,
        guild_id: i32,
        user_id: i32,
        pool: &Pool<Postgres>,
    ) -> Result<Option<Vec<Channel>>, Box<dyn std::error::Error>> {
        match sqlx::query_as::<_, Channel>(
            "SELECT c.id, c.name, c.message_count, c.guild_id, c.created_date, c.updated_date
                FROM channel c
                JOIN member m on c.guild_id = m.guild_id
                WHERE m.user_id = $1 AND c.guild_id = $2",
        )
        .bind(user_id)
        .bind(guild_id)
        .fetch_all(pool)
        .await
        {
            Ok(result) => Ok(Some(result)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn find_channel(
        &self,
        channel_id: i32,
        guild_id: i32,
        user_id: i32,
        pool: &Pool<Postgres>,
    ) -> Result<Option<Channel>, Box<dyn std::error::Error>> {
        match sqlx::query_as::<_, Channel>(
            "SELECT c.id, c.name, c.message_count, c.guild_id, c.created_date, c.updated_date
            FROM channel c
            JOIN member m on c.guild_id = m.guild_id
            WHERE c.id = $1 AND m.user_id = $2 AND c.guild_id = $3"
        )
        .bind(channel_id)
        .bind(user_id)
        .bind(guild_id)
        .fetch_one(pool)
        .await
        {
            Ok(result) => Ok(Some(result)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
