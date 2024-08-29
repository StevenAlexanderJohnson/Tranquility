use sqlx::{Pool, Postgres};

use crate::Guild;

pub struct GuildRepository {
    pool: Pool<Postgres>,
}

impl GuildRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(
        &self,
        id: i32,
        member_id: i32,
    ) -> Result<Option<Guild>, Box<dyn std::error::Error>> {
        match sqlx::query_as::<_, Guild>(
            "
            SELECT g.id, g.name, g.description, g.owner_id, g.created_date, g.updated_date
            FROM guild g
            JOIN member as m on m.guild_id = g.id AND m.user_id = $2
            WHERE g.id = $1;
        ",
        )
        .bind(id)
        .bind(member_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.into())
        {
            Ok(guild) => Ok(Some(guild)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn find_guilds(
        &self,
        id: i32,
    ) -> Result<Option<Vec<Guild>>, Box<dyn std::error::Error>> {
        match sqlx::query_as(
            "
            SELECT g.id, g.name, g.description, g.owner_id, g.created_date, g.updated_date
            FROM guild g
            JOIN member m on m.guild_id = g.id
            WHERE m.user_id = $1;"
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await {
            Ok(guild) => Ok(Some(guild)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e.into())
        }
    }

    pub async fn find_owner_guilds(
        &self,
        id: i32,
    ) -> Result<Option<Vec<Guild>>, Box<dyn std::error::Error>> {
        match sqlx::query_as(
            "
            SELECT id, name, description, owner_id, created_date, updated_date
            FROM guild
            WHERE owner_id = $1;
        ",
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        {
            Ok(guild) => Ok(Some(guild)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn insert(&self, guild: &Guild) -> Result<Guild, Box<dyn std::error::Error>> {
        sqlx::query_as::<_, Guild>(
            "
            INSERT INTO guild (name, description, owner_id)
            VALUES ($1, $2, $3)
            RETURNING id, name, description, owner_id, created_date, updated_date;
            ",
        )
        .bind(guild.name.to_string())
        .bind(guild.description.to_string())
        .bind(guild.owner_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.into())
    }
}
