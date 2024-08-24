use actix_web::{
    cookie::{
        time::{Duration, OffsetDateTime},
        Cookie,
    },
    post, web, HttpResponse,
};
use server::auth_user::AuthUser;

use crate::{data_access::auth_repository::AuthRepository, jwt_handler::generate_token};

#[post("/login")]
pub async fn login(
    auth_user: web::Json<AuthUser>,
    repository: web::Data<AuthRepository>,
) -> HttpResponse {
    match repository.find(auth_user.into_inner()).await {
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
            HttpResponse::Ok().cookie(cookie).finish()
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
    repository: web::Data<AuthRepository>,
) -> HttpResponse {
    if auth_user.email.is_none() || auth_user.claims.is_some() {
        return HttpResponse::BadRequest().finish();
    }
    match repository.insert(auth_user.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to register: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
