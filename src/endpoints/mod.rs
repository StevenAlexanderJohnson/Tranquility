mod guild;
mod websocket;
mod auth;

pub fn auth_endpoints() -> actix_web::Scope {
    actix_web::web::scope("/auth").service(auth::login)
}

pub fn websocket_endpoints() -> actix_web::Scope {
    actix_web::web::scope("/ws").service(websocket::echo)
}

pub fn guild_endpoints() -> actix_web::Scope {
    actix_web::web::scope("/guild")
        .service(guild::get_guilds)
        .service(guild::get_guild)
        .service(guild::create_guild)
}
