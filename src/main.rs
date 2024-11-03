mod auth_middleware;
mod endpoints;
mod jwt_handler;
mod password_manager;

use actix_cors::Cors;
use actix_web::{
    dev::Service,
    middleware::Logger,
    web::{scope, Data},
    App, HttpServer,
};
use auth_middleware::Auth;
use data_access::DatabaseConnection;
use env_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Connecting to the database...");
    let data_access = DatabaseConnection::new(data_access::create_connection_pool(32).await);
    println!("Connected to the database\n");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(Logger::new("%r %s %{User-Agent}i"))
            .wrap(cors)
            .wrap_fn(|req, srv| {
                println!("Incoming request to: {}", req.path());
                println!("Headers: {:?}", req.headers());
                srv.call(req)
            })
            .app_data(Data::new(data_access.clone()))
            .service(
                scope("/api")
                    .wrap(Auth)
                    .service(endpoints::auth_endpoints())
                    .service(endpoints::guild_endpoints()),
            )
            .service(scope("/ws").service(endpoints::websocket_endpoints()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
