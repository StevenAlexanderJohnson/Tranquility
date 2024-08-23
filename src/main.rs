mod auth_middleware;
mod data_access;
mod endpoints;
mod jwt_handler;

use actix_web::{
    web::{self, scope},
    App, HttpServer,
};
use data_access::{auth_repository::AuthRepository, guild_repository::GuildRepository};
use mongodb::Client;
use server::{auth_user::AuthUser, guild::Guild};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let uri =
        std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    println!("Connecting to MongoDB at {}", uri);
    let client = Client::with_uri_str(uri)
        .await
        .expect("Failed to connect to MongoDB");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(GuildRepository::new(
                client.database("tranquility").collection::<Guild>("guilds"),
            )))
            .app_data(web::Data::new(AuthRepository::new(
                client
                    .database("tranquility")
                    .collection::<AuthUser>("auth"),
            )))
            .service(endpoints::websocket_endpoints())
            .service(endpoints::auth_endpoints())
            .service(
                scope("/api")
                    .wrap(auth_middleware::Auth)
                    .service(endpoints::guild_endpoints()),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
