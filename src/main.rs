mod auth_middleware;
mod endpoints;
mod file_handler;
mod jwt_handler;
mod password_manager;

use std::io::ErrorKind;

use actix_cors::Cors;
use actix_web::{
    dev::Service, middleware::Logger, rt::spawn, web::{scope, Data}, App, HttpServer
};
use auth_middleware::Auth;
use data_access::DatabaseConnection;
use file_handler::LocalFileHandler;
use log::{self, info};
use tokio::try_join;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("Connecting to the database...");
    let data_access = DatabaseConnection::new(data_access::create_connection_pool(32).await);
    info!("Connected to the database\n");

    info!("Checking upload destination...");
    let file_handler = LocalFileHandler::new();
    file_handler
        .check_destination()
        .map_err(|e| std::io::Error::new(ErrorKind::InvalidInput, e.to_string()))?;
    info!("File destination is valid and ready\n");
    

    let (chat_server, server_tx) = notifications::WebsocketServer::new();
    let chat_server = spawn(chat_server.run());

    let http_server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(Logger::new("%r %s %{User-Agent}i"))
            .wrap(cors)
            .wrap_fn(|req, srv| srv.call(req))
            .app_data(Data::new(server_tx.clone()))
            .app_data(Data::new(data_access.clone()))
            .app_data(Data::new(file_handler.clone()))
            .service(
                scope("/api")
                    .wrap(Auth)
                    .service(endpoints::auth_endpoints())
                    .service(endpoints::guild_endpoints())
                    .service(endpoints::attachment_endpoints()),
            )
            .service(scope("/ws").service(endpoints::websocket_endpoints()))
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    try_join!(http_server, async move { chat_server.await.unwrap() })?;

    Ok(())
}
