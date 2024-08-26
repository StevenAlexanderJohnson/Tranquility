use actix_web::{
    cookie::{
        time::{Duration, OffsetDateTime},
        Cookie,
    },
    post, web, HttpResponse,
};
// use server::{auth_user::AuthUser, member::Member};
use data_access::AuthUser;

use crate::jwt_handler::generate_token;

#[post("/login")]
pub async fn login(
    auth_user: web::Json<AuthUser>,
    repository: web::Data<data_access::AuthRepository>,
) -> HttpResponse {
    let user = auth_user.into_inner();
    match repository.find(user).await {
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
    repository: web::Data<data_access::AuthRepository>,
) -> HttpResponse {
    if auth_user.email.is_none() {
        return HttpResponse::BadRequest().finish();
    }

    let inserted_user = match repository.insert(&auth_user.into_inner()).await {
        Ok(user) => user,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().json(inserted_user)
}
