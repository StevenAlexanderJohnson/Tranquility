use actix_web::{
    web::{self, scope},
    App, HttpServer,
};

use mongodb::Client;
use server::guild::Guild;
mod auth_middleware;
mod data_access;
mod endpoints;
use data_access::guild_repository::GuildRepository;

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
            .service(endpoints::websocket_endpoints())
            .service(scope("/auth").service(endpoints::auth_endpoints()))
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
