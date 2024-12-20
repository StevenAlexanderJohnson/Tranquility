mod attachments;
mod auth;
mod channel;
mod guilds;
mod members;
mod messages;
mod roles;

use auth::auth_repository::AuthRepository;
pub use auth::model::AuthUser;

use guilds::guild_repository::GuildRepository;
pub use guilds::model::Guild;

use channel::channel_repository::ChannelRepository;
pub use channel::model::Channel;

use members::member_repository::MemberRepository;
pub use members::model::Member;

pub use roles::model::{Role, RoleResult};
use roles::{model::Intent, role_repository::RoleRepository};

use messages::message_repository::MessageRepository;
pub use messages::model::Message;

use attachments::attachments_repository::AttachmentsRepository;
pub use attachments::model::{Attachment, AttachmentMapping};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use data_models::{CreateChannelResponse, CreateGuildResponse};

use data_models::{
    AttachmentResponse, AuthUserResponse, CreateAuthUserRequest, CreateChannelRequest,
    CreateGuildRequest, CreateMemberRequest, CreateMessageRequest, CreateRoleRequest,
    MessageResponse,
};

/// Creates a connection pool to the database
///
/// # Notes
///
/// This function will panic if the `POSTGRES_URI` environment variable is not set.
/// It uses this value in order to connect to the database so that it doesn't have to be passed in.
///
/// # Arguments
///
/// * `max_connections` - The maximum number of connections to the database
///
/// # Returns
///
/// A connection pool to the database - `Pool<Postgres>`
pub async fn create_connection_pool(max_connections: u32) -> Pool<Postgres> {
    let uri = std::env::var("POSTGRES_URI").expect("POSTGRES_URI is not set");

    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&uri)
        .await
        .expect("Unable to create connection pool")
}

/// A struct that holds the database connection and repositories.
///
/// # Fields
///
/// * `pool` - The connection pool to the database.
/// * `auth` - The repository for the `AuthUser` model.
/// * `channel` - The repository for the `Channel` model.
/// * `guild` - The repository for the `Guild` model.
/// * `member` - The repository for the `Member` model.
///
/// # Notes
///
/// This struct is used to interact with the database and abstracts the database connection and repositories.
#[derive(Clone)]
pub struct DatabaseConnection {
    pool: Pool<Postgres>,
    auth: Box<AuthRepository>,
    channel: Box<ChannelRepository>,
    guild: Box<GuildRepository>,
    member: Box<MemberRepository>,
    role: Box<RoleRepository>,
    message: Box<MessageRepository>,
    attachment: Box<AttachmentsRepository>,
}

impl DatabaseConnection {
    /// Creates a new `DatabaseConnection` struct
    ///
    /// # Arguments
    ///
    /// * `pool` - The connection pool to the database.
    ///
    /// # Returns
    ///
    /// A new `DatabaseConnection` struct
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {
            pool,
            auth: Box::new(AuthRepository {}),
            channel: Box::new(ChannelRepository {}),
            guild: Box::new(GuildRepository {}),
            member: Box::new(MemberRepository {}),
            role: Box::new(RoleRepository {}),
            message: Box::new(MessageRepository {}),
            attachment: Box::new(AttachmentsRepository {}),
        }
    }

    /// Registers a user in the auth table in the database.
    ///
    /// # Arguments
    ///
    /// * `user` - The user to register.
    ///
    /// # Returns
    ///
    /// A result containing the registered user or an error.
    /// The returned user will have the 'id' field set as well as the `created_date` and `updated_date` fields.
    pub async fn register_user(
        &self,
        user: &CreateAuthUserRequest,
    ) -> Result<AuthUser, Box<dyn std::error::Error>> {
        self.auth
            .insert(user, &self.pool)
            .await
            .map_err(|e| e.into())
    }

    /// Logs a user in by checking the auth table in the database.
    ///
    /// # Arguments
    ///
    /// * `user` - The user to log in.
    ///
    /// # Returns
    ///
    /// A result containing the logged in user or an error.
    /// The returned user will only have the `id`, `username`, `email`, and `refresh_token` fields set.
    pub async fn login(
        &self,
        user: &AuthUser,
    ) -> Result<Option<AuthUser>, Box<dyn std::error::Error>> {
        let mut tx = self.pool.begin().await?;
        match self.auth.find(user, &mut tx).await {
            Ok(x) => {
                tx.commit().await?;
                Ok(x)
            }
            Err(e) => {
                tx.rollback().await?;
                Err(e)
            }
        }
    }

    pub async fn websocket_login(
        &self,
        id: i32,
        websocket_token: &str,
    ) -> Result<AuthUserResponse, Box<dyn std::error::Error>> {
        let mut tx = self.pool.begin().await?;
        let user = match self.auth.find_websocket(id, websocket_token, &mut tx).await {
            Ok(Some(x)) => {
                tx.commit().await?;
                x
            }
            Ok(None) => {
                tx.rollback().await?;
                return Err(Box::from("User not found"));
            }
            Err(e) => {
                tx.rollback().await?;
                return Err(e);
            }
        };

        Ok(AuthUserResponse::try_from(user)?)
    }

    /// Generates a new refresh token.
    ///
    /// # Arguments
    ///
    /// * `user_id` - ID of user requesting a refresh.
    ///
    /// # Returns
    ///
    /// A result containing an option of the updated AuthUser struct or an error.
    /// In Ok(user), if user is None that means the provided user_id and provided refresh_token is incorrect.
    pub async fn refresh_auth_token(
        &self,
        user_id: i32,
        refresh_token: String,
    ) -> Result<Option<AuthUser>, Box<dyn std::error::Error>> {
        let mut tx = self.pool.begin().await?;
        match self
            .auth
            .update_refresh_token(user_id, refresh_token, &mut tx)
            .await
        {
            Ok(x) => {
                tx.commit().await?;
                Ok(x)
            }
            Err(e) => {
                tx.rollback().await?;
                Err(e)
            }
        }
    }

    /// Finds all the guilds that the user has joined.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The id of the user to find the guilds for.
    ///
    /// # Returns
    ///
    /// A result containing the guilds that the user has joined or an error.
    /// The list of guilds can also be None if the user has not joined any guilds.
    pub async fn find_joined_guild(
        &self,
        user_id: i32,
    ) -> Result<Option<Vec<CreateGuildResponse>>, Box<dyn std::error::Error>> {
        match self.guild.find_guilds(user_id, &self.pool).await {
            Ok(Some(guilds)) => {
                let mut guild_response = guilds
                    .into_iter()
                    .map(CreateGuildResponse::try_from)
                    .collect::<Result<Vec<_>, _>>()?;

                for guild in &mut guild_response {
                    let channels: Vec<CreateChannelResponse> =
                        match self.find_guild_channels(guild.id, user_id).await {
                            Ok(Some(channels)) => channels,
                            Ok(None) => vec![],
                            Err(e) => {
                                println!("{:?}", e);
                                return Err("".into());
                            }
                        };
                    guild.channels = channels;
                }

                Ok(Some(guild_response))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Finds all the guilds that the user owns.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The id of the user to find the guilds for.
    ///
    /// # Returns
    ///
    /// A result containing the guilds that the user owns or an error.
    /// The list of guilds can also be None if the user does not own any guilds.
    pub async fn find_owned_guilds(
        &self,
        user_id: i32,
    ) -> Result<Option<Vec<CreateGuildResponse>>, Box<dyn std::error::Error>> {
        match self.guild.find_owner_guilds(user_id, &self.pool).await {
            Ok(Some(guild)) => {
                let mut guilds = guild
                    .into_iter()
                    .map(CreateGuildResponse::try_from)
                    .collect::<Result<Vec<_>, _>>()?;
                for guild in &mut guilds {
                    guild.channels = match self.find_guild_channels(guild.id, user_id).await {
                        Ok(channels) => channels.unwrap_or_default(),
                        Err(e) => {
                            println!("{:?}", e);
                            return Err("".into());
                        }
                    };
                }

                Ok(Some(guilds))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Finds a guild by its id.
    ///
    /// # Arguments
    ///
    /// * `guild_id` - The id of the guild to find.
    /// * `member_id` - The id of the member to find the guild for.
    ///
    /// # Notes
    ///
    /// The `member_id` is required because guilds will be hidden by default.
    /// Only invited members can view or search for the guild.
    ///
    /// # Returns
    ///
    /// A result containing the guild or an error.
    /// The guild can also be None if the guild does not exist.
    pub async fn find_guild_by_id(
        &self,
        guild_id: i32,
        member_id: i32,
    ) -> Result<Option<CreateGuildResponse>, Box<dyn std::error::Error>> {
        let mut guild: CreateGuildResponse =
            match self.guild.find_by_id(guild_id, member_id, &self.pool).await {
                Ok(Some(guild)) => guild.try_into(),
                Ok(None) => {
                    return Ok(None);
                }
                Err(e) => {
                    return Err(e);
                }
            }?;

        guild.channels = match self.find_guild_channels(guild_id, member_id).await? {
            Some(channels) => channels,
            None => vec![],
        };

        Ok(Some(guild))
    }

    /// Creates a new guild in the database.
    ///
    /// # Notes
    ///
    /// The owner of the guild should be set before trying to create it.
    /// This value will be used to add the owner to the guild in the `member` table.
    ///
    /// # Arguments
    ///
    /// * `guild` - The guild to create.
    ///
    /// # Returns
    ///
    /// A result containing the created guild or an error.
    /// The created guild will have the `id` field set as well as the `created_date` and `updated_date` fields.
    pub async fn create_guild(
        &self,
        guild: &CreateGuildRequest,
        owner_id: i32,
    ) -> Result<CreateGuildResponse, Box<dyn std::error::Error>> {
        let mut tx = self.pool.begin().await?;

        let guild = match self.guild.insert(guild, owner_id, &mut tx).await {
            Ok(guild) => guild,
            Err(e) => {
                tx.rollback().await?;
                return Err(e);
            }
        };

        if let Err(e) = self
            .member
            .add_user_to_guild(
                &CreateMemberRequest {
                    guild_id: guild.id.unwrap(),
                    user_id: guild.owner_id.unwrap(),
                },
                guild.owner_id.unwrap(),
                &mut tx,
            )
            .await
        {
            println!("{:?}", e);
            tx.rollback().await?;
            return Err(e);
        }

        tx.commit().await?;

        Ok(CreateGuildResponse::try_from(guild)?)
    }

    pub async fn find_guild_channels(
        &self,
        guild_id: i32,
        user_id: i32,
    ) -> Result<Option<Vec<CreateChannelResponse>>, Box<dyn std::error::Error>> {
        self.channel
            .find_guild_channels(guild_id, user_id, &self.pool)
            .await
            .map(|opt_channels| {
                opt_channels
                    .map(|channels| {
                        channels
                            .into_iter()
                            .map(|channel| CreateChannelResponse::try_from(channel))
                            .collect::<Result<Vec<CreateChannelResponse>, _>>()
                    })
                    .transpose()
            })?
    }

    pub async fn find_channel(
        &self,
        channel_id: i32,
        guild_id: i32,
        user_id: i32,
    ) -> Result<Option<CreateChannelResponse>, Box<dyn std::error::Error>> {
        self.channel
            .find_channel(channel_id, guild_id, user_id, &self.pool)
            .await
            .and_then(|option| option.map(CreateChannelResponse::try_from).transpose())
    }

    pub async fn create_guild_channel(
        &self,
        channel: &CreateChannelRequest,
        guild_id: i32,
        user_id: i32,
    ) -> Result<Option<CreateChannelResponse>, Box<dyn std::error::Error>> {
        let mut tx = self.pool.begin().await?;
        match self
            .channel
            .insert(channel, guild_id, user_id, &mut tx)
            .await
            .and_then(|option| option.map(CreateChannelResponse::try_from).transpose())
        {
            Ok(x) => {
                tx.commit().await?;
                Ok(x)
            }
            Err(e) => {
                tx.rollback().await?;
                Err(e)
            }
        }
    }

    pub async fn create_guild_role(
        &self,
        role: &CreateRoleRequest,
        user_id: i32,
    ) -> Result<Option<RoleResult>, Box<dyn std::error::Error>> {
        let mut tx = self.pool.begin().await?;
        let new_role: Role = match self
            .role
            .create_role(role.guild_id, &role.name, &user_id, &mut tx)
            .await
        {
            Ok(x) => x,
            Err(e) => {
                tx.rollback().await?;
                println!("{:?}", e);
                return Err(e);
            }
        };

        let mut new_intents = Vec::<Intent>::new();
        for &intent in role.intents.iter() {
            match self
                .role
                .add_row_intent(new_role.id.unwrap(), intent as i32, &user_id, &mut tx)
                .await
            {
                Ok(x) => new_intents.push(x),
                Err(e) => {
                    tx.rollback().await?;
                    println!("{:?}", e);
                    return Err(e);
                }
            }
        }

        Ok(Some(RoleResult {
            id: new_role.id,
            name: new_role.name,
            guild_id: new_role.guild_id,
            intents: new_intents,
            created_date: new_role.created_date,
            updated_date: new_role.updated_date,
        }))
    }

    pub async fn create_message(
        &self,
        message: &CreateMessageRequest,
        user_id: i32,
    ) -> Result<MessageResponse, Box<dyn std::error::Error>> {
        let mut tx = self.pool.begin().await?;

        let mut output = match self.message.insert(message, user_id, &mut tx).await {
            Ok(x) => MessageResponse::try_from(x)?,
            Err(e) => {
                tx.rollback().await?;
                return Err(e);
            }
        };

        for &attachment_id in &message.attachments {
            match self
                .attachment
                .create_message_attachment_mapping(
                    &AttachmentMapping {
                        post_id: output.id,
                        attachment_id: attachment_id,
                    },
                    &mut tx,
                )
                .await
            {
                Ok(()) => {}
                Err(e) => {
                    tx.rollback().await?;
                    return Err(e);
                }
            };
        }

        if message.attachments.len() > 0 {
            // output.attachments = self.get_post_attachment(output.id).await.iter();
            let attachments = self
                .attachment
                .get_message_attachments(output.id, &mut tx)
                .await?
                .iter()
                .map(|x| {
                    x.file_name
                        .to_owned()
                        .ok_or("Unable to get file name when submitting message with attachments.")
                })
                .collect::<Result<Vec<String>, _>>()?;
            output.attachments = attachments;
        }

        tx.commit().await?;
        Ok(output)
    }

    pub async fn get_channel_message(
        &self,
        guild_id: i32,
        channel_id: i32,
        user_id: i32,
        page_offset: i32,
    ) -> Result<Option<Vec<MessageResponse>>, Box<dyn std::error::Error>> {
        let page_size = 20;
        println!("User: {}, Guild: {}, Channel: {}, Offset: {}, Page_Number: {}", user_id, guild_id, channel_id, page_size, page_offset);
        self.message
            .get_page(
                page_size,
                page_offset,
                user_id,
                guild_id,
                channel_id,
                &self.pool,
            )
            .await
            .map(|message_options| {
                message_options
                    .map(|messages| {
                        messages
                            .into_iter()
                            .map(MessageResponse::try_from)
                            .collect::<Result<Vec<MessageResponse>, _>>()
                    })
                    .transpose()
            })?
    }

    pub async fn create_attachment(
        &self,
        attachment: &Attachment,
        user_id: i32,
    ) -> Result<Option<AttachmentResponse>, Box<dyn std::error::Error>> {
        let mut tx = self.pool.begin().await?;
        match self.attachment.insert(attachment, user_id, &mut tx).await {
            Ok(Some(x)) => {
                tx.commit().await?;
                Ok(Some(AttachmentResponse::try_from(&x)?))
            }
            Ok(None) => {
                tx.rollback().await?;
                Err("An error occurred while trying to create attachment in the database".into())
            }
            Err(e) => {
                tx.rollback().await?;
                Err(e)
            }
        }
    }

    pub async fn get_post_attachment(
        &self,
        post_id: i32,
    ) -> Result<Vec<AttachmentResponse>, Box<dyn std::error::Error>> {
        let mut tx = self.pool.begin().await?;
        let output = self
            .attachment
            .get_message_attachments(post_id, &mut tx)
            .await?
            .iter()
            .map(|x| AttachmentResponse::try_from(x))
            .collect::<Result<Vec<AttachmentResponse>, _>>();
        println!("GET POST: {} {:?}", post_id, output);
        output
    }
}
