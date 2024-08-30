mod auth;
mod channel;
mod guilds;
mod members;
mod roles;

use auth::auth_repository::AuthRepository;
pub use auth::model::AuthUser;

use guilds::guild_repository::GuildRepository;
pub use guilds::model::Guild;

use channel::channel_repository::ChannelRepository;
pub use channel::model::Channel;

use members::member_repository::MemberRepository;
pub use members::model::Member;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn create_connection_pool(max_connections: u32) -> Pool<Postgres> {
    let uri = std::env::var("POSTGRES_URI").expect("POSTGRES_URI is not set");
    println!("{}", uri);

    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&uri)
        .await
        .expect("Unable to create connection pool")
}

#[derive(Clone)]
pub struct DatabaseConnection {
    pool: Pool<Postgres>,
    auth: Box<AuthRepository>,
    channel: Box<ChannelRepository>,
    guild: Box<GuildRepository>,
    member: Box<MemberRepository>,
}

impl DatabaseConnection {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {
            pool,
            auth: Box::new(AuthRepository {}),
            channel: Box::new(ChannelRepository {}),
            guild: Box::new(GuildRepository {}),
            member: Box::new(MemberRepository {}),
        }
    }

    pub async fn register_user(
        &self,
        user: &AuthUser,
    ) -> Result<AuthUser, Box<dyn std::error::Error>> {
        self.auth
            .insert(user, &self.pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn login(
        &self,
        user: &AuthUser,
    ) -> Result<Option<AuthUser>, Box<dyn std::error::Error>> {
        self.auth.find(user, &self.pool).await.map_err(|e| e.into())
    }

    pub async fn find_joined_guild(
        &self,
        user_id: i32,
    ) -> Result<Option<Vec<Guild>>, Box<dyn std::error::Error>> {
        self.guild.find_guilds(user_id, &self.pool).await
    }

    pub async fn find_owned_guilds(
        &self,
        user_id: i32,
    ) -> Result<Option<Vec<Guild>>, Box<dyn std::error::Error>> {
        self.guild.find_owner_guilds(user_id, &self.pool).await
    }

    pub async fn find_guild_by_id(
        &self,
        guild_id: i32,
        member_id: i32,
    ) -> Result<Option<Guild>, Box<dyn std::error::Error>> {
        self.guild.find_by_id(guild_id, member_id, &self.pool).await
    }

    pub async fn create_guild(&self, guild: &Guild) -> Result<Guild, Box<dyn std::error::Error>> {
        let mut tx = self.pool.begin().await?;

        let guild = match self.guild.insert(guild, &mut tx).await {
            Ok(guild) => guild,
            Err(e) => {
                tx.rollback().await?;
                return Err(e);
            }
        };

        if let Err(e) = self.member.add_user_to_guild(guild.owner_id.unwrap(), guild.id.unwrap(), guild.owner_id.unwrap(), &mut tx).await {
            println!("{:?}", e);
            tx.rollback().await?;
            return Err(e);
        }

        Ok(guild)
    }
}
