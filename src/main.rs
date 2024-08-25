mod auth_middleware;
mod endpoints;
mod jwt_handler;

use actix_web::{web::Data, App, HttpServer};
use data_access::AuthRepository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection_pool = data_access::create_connection_pool(5).await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AuthRepository::new(connection_pool.clone())))
            .service(endpoints::auth_endpoints())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
