mod auth_middleware;
mod endpoints;
mod jwt_handler;
mod password_manager;

use actix_web::{
    web::{scope, Data},
    App, HttpServer,
};
use auth_middleware::Auth;
use data_access::DatabaseConnection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Connecting to the database...");
    let data_access = DatabaseConnection::new(data_access::create_connection_pool(32).await);
    println!("Connected to the database\n");

    HttpServer::new(move || {
        App::new()
            .wrap(Auth)
            .app_data(Data::new(data_access.clone()))
            .service(endpoints::websocket_endpoints())
            .service(endpoints::auth_endpoints())
            .service(scope("/api").service(endpoints::guild_endpoints()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
