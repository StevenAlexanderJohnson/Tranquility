mod auth;
mod guilds;
mod channel;
mod roles;

pub use auth::auth_repository::AuthRepository;
pub use auth::model::AuthUser;

pub use guilds::guild_repository::GuildRepository;
pub use guilds::model::Guild;

pub use channel::channel_repository::ChannelRepository;
pub use channel::model::Channel;

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
