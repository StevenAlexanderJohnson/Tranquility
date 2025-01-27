mod attachment;
mod auth;
mod guild;
mod message;
mod websocket;

pub fn auth_endpoints() -> actix_web::Scope {
    actix_web::web::scope("/auth")
        .service(auth::login)
        .service(auth::register)
        .service(auth::refresh_token)
}

pub fn websocket_endpoints() -> actix_web::Scope {
    actix_web::web::scope("").service(websocket::gateway)
}

pub fn attachment_endpoints() -> actix_web::Scope {
    actix_web::web::scope("/attachment").service(attachment::upload_attachments)
}

pub fn guild_endpoints() -> actix_web::Scope {
    actix_web::web::scope("/guild")
        .service(guild::get_owned_guilds)
        .service(guild::get_guild)
        .service(guild::get_guilds)
        .service(guild::create_guild)
        .service(guild::create_guild_channel)
        .service(guild::get_guild_channels)
        .service(guild::get_guild_channel)
        .service(guild::create_guild_role)
        .service(message::get_channel_messages)
}
