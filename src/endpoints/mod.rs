mod auth;
mod guild;
mod websocket;

pub fn auth_endpoints() -> actix_web::Scope {
    actix_web::web::scope("/auth")
        .service(auth::login)
        .service(auth::register)
        .service(auth::refresh_token)
}

pub fn websocket_endpoints() -> actix_web::Scope {
    actix_web::web::scope("/ws").service(websocket::echo)
}

pub fn guild_endpoints() -> actix_web::Scope {
    actix_web::web::scope("/guild")
        .service(guild::get_owned_guilds)
        .service(guild::get_guild)
        .service(guild::get_guilds)
        .service(guild::create_guild)
    //         .service(guild::create_channel)
}
