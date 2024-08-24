use std::collections::HashSet;

use actix_web::{
    cookie::{
        time::{Duration, OffsetDateTime},
        Cookie,
    },
    post, web, HttpResponse,
};
use server::{auth_user::AuthUser, member::Member};

use crate::{
    data_access::{auth_repository::AuthRepository, member_repository::MemberRepository},
    jwt_handler::generate_token,
};

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
    member_repository: web::Data<MemberRepository>,
) -> HttpResponse {
    if auth_user.email.is_none() || auth_user.claims.is_some() {
        return HttpResponse::BadRequest().finish();
    }

    let new_id = match repository.insert(auth_user.clone()).await {
        Ok(id) => id,
        Err(e) => {
            println!("Failed to register: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    match member_repository
        .insert(Member {
            auth_id: new_id.as_object_id().unwrap(),
            id: None,
            friends: HashSet::new(),
            name: auth_user.into_inner().username,
            joined_date: chrono::Utc::now(),
        })
        .await
    {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
