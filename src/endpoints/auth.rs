use actix_web::{
    cookie::{
        time::{Duration, OffsetDateTime},
        Cookie,
    },
    post, web, HttpResponse,
};
use data_access::{AuthUser, DatabaseConnection};

use data_models::CreateAuthUserRequest;

use crate::password_manager::hash_password;
use crate::{
    jwt_handler::{generate_token, Claims},
    password_manager::validate_password,
};

#[post("/login")]
pub async fn login(
    auth_user: web::Json<AuthUser>,
    repository: web::Data<DatabaseConnection>,
) -> HttpResponse {
    if auth_user.password.is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let auth_user = auth_user.into_inner();

    match repository.login(&auth_user).await {
        Ok(Some(user)) => {
            if let Err(e) = validate_password(
                auth_user.password.as_ref().unwrap(),
                user.password.as_ref().unwrap(),
            ) {
                println!("Unable to authenticate user via password: {:?}", e);
                return HttpResponse::Unauthorized().finish();
            }

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
            HttpResponse::Ok().cookie(cookie).json(AuthUser {
                password: None,
                ..user
            })
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
    mut auth_user: web::Json<CreateAuthUserRequest>,
    repository: web::Data<DatabaseConnection>,
) -> HttpResponse {
    auth_user.password = match hash_password(&auth_user.password) {
        Ok(password) => password,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

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
    data: web::ReqData<Claims>,
    token: web::Path<String>,
) -> HttpResponse {
    let auth_user = match repository
        .refresh_auth_token(data.id, token.into_inner())
        .await
    {
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
