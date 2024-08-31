use std::collections::BTreeMap;

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
                .expires(OffsetDateTime::now_utc().checked_add(Duration::minutes(10)))
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

#[post("/refresh/{token}")]
pub async fn refresh_token(
    repository: web::Data<DatabaseConnection>,
    data: web::ReqData<BTreeMap<String, String>>,
    token: web::Path<String>,
) -> HttpResponse {
    let id = match data.get("id").and_then(|id| id.parse::<i32>().ok()) {
        Some(x) => x,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let auth_user = match repository.refresh_auth_token(id, token.into_inner()).await {
        Ok(Some(auth_user)) => auth_user,
        Ok(None) => return HttpResponse::Unauthorized().finish(),
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::Unauthorized().finish();
        }
    };
    let jwt = match generate_token(&auth_user) {
        Ok(jwt) => jwt,
        Err(e) => {
            println!("Failed to generate token: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };
    let cookie = Cookie::build("auth_token", jwt)
        .domain("localhost")
        .path("/")
        .expires(OffsetDateTime::now_utc().checked_add(Duration::minutes(10)))
        .finish();

    HttpResponse::Ok().cookie(cookie).json(auth_user)
}
