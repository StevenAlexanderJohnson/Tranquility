mod auth_middleware;
mod endpoints;
mod jwt_handler;

use actix_web::{
    web::{scope, Data},
    App, HttpServer,
};
use auth_middleware::Auth;
use data_access::{AuthRepository, GuildRepository};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection_pool = data_access::create_connection_pool(5).await;

    HttpServer::new(move || {
        App::new()
            .wrap(Auth)
            .app_data(Data::new(AuthRepository::new(connection_pool.clone())))
            .app_data(Data::new(GuildRepository::new(connection_pool.clone())))
            .service(endpoints::websocket_endpoints())
            .service(endpoints::auth_endpoints())
            .service(scope("/api").service(endpoints::guild_endpoints()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
