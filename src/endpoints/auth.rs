use actix_web::{
    cookie::{
        time::{Duration, OffsetDateTime},
        Cookie,
    },
    post, web, HttpResponse,
};
use data_access::{AuthUser, DatabaseConnection};

use crate::jwt_handler::generate_token;

#[post("/login")]
pub async fn login(
    auth_user: web::Json<AuthUser>,
    repository: web::Data<DatabaseConnection>,
) -> HttpResponse {
    match repository.login(&auth_user.into_inner()).await {
        Ok(Some(user)) => {
            let jwt = match generate_token(&user) {
                Ok(jwt) => jwt,
                Err(e) => {
                    println!("Failed to generate token: {}", e);
                    return HttpResponse::InternalServerError().finish();
                }
            };
            let cookie = Cookie::build("auth_token", jwt)
                .domain("localhost")
                .path("/")
                .expires(OffsetDateTime::now_utc().checked_add(Duration::minutes(2)))
                .finish();
            HttpResponse::Ok().cookie(cookie).json(user)
        }
        Ok(None) => HttpResponse::Unauthorized().finish(),
        Err(e) => {
            println!("Failed to login: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/register")]
pub async fn register(
    auth_user: web::Json<AuthUser>,
    repository: web::Data<DatabaseConnection>,
) -> HttpResponse {
    if auth_user.email.is_none() {
        return HttpResponse::BadRequest().finish();
    }

    match repository.register_user(&auth_user.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
