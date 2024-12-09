use actix_web::{get, post, web, HttpResponse, ResponseError};
use data_access::{DatabaseConnection, Guild};
use data_models::{
    CreateChannelRequest, CreateChannelResponse, CreateGuildRequest, CreateRoleRequest,
};
use log::error;

use crate::jwt_handler::Claims;

#[get("/")]
pub async fn get_guilds(
    claims: web::ReqData<Claims>,
    repository: web::Data<DatabaseConnection>,
) -> HttpResponse {
    match repository.find_joined_guild(claims.id).await {
        Ok(Some(guilds)) => HttpResponse::Ok().json(guilds),
        Ok(None) => HttpResponse::Ok().json(Vec::<Guild>::new()),
        Err(e) => {
            println!("{e:?}");
            e.error_response()
        }
    }
}

#[get("/owned")]
pub async fn get_owned_guilds(
    claims: web::ReqData<Claims>,
    repository: web::Data<DatabaseConnection>,
) -> HttpResponse {
    match repository.find_owned_guilds(claims.id).await {
        Ok(guilds) => HttpResponse::Ok().json(guilds),
        Err(e) => {
            println!("Failed to get guilds: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/{guild_id}")]
pub async fn get_guild(
    repository: web::Data<DatabaseConnection>,
    claims: web::ReqData<Claims>,
    path: web::Path<i32>,
) -> HttpResponse {
    match repository
        .find_guild_by_id(path.into_inner(), claims.id)
        .await
    {
        Ok(Some(guild)) => HttpResponse::Ok().json(guild),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => {
            println!("Failed to get guild: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/")]
pub async fn create_guild(
    repository: web::Data<DatabaseConnection>,
    guild: web::Json<CreateGuildRequest>,
    claims: web::ReqData<Claims>,
) -> HttpResponse {
    match repository.create_guild(&guild, claims.id).await {
        Ok(guild) => HttpResponse::Created().json(guild),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/{guild_id}/channel")]
pub async fn get_guild_channels(
    repository: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
    claims: web::ReqData<Claims>,
) -> HttpResponse {
    let channels: Result<Option<Vec<CreateChannelResponse>>, _> = repository
        .find_guild_channels(path.into_inner(), claims.id)
        .await
        .and_then(|option| {
            option
                .map(|channels| {
                    channels
                        .into_iter()
                        .map(CreateChannelResponse::try_from)
                        .collect()
                })
                .transpose()
        });

    match channels {
        Ok(Some(channels)) => HttpResponse::Ok().json(channels),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("Error collecting user's channels: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/{guild_id}/channel")]
pub async fn create_guild_channel(
    repository: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
    claims: web::ReqData<Claims>,
    channel: web::Json<CreateChannelRequest>,
) -> HttpResponse {
    match repository
        .create_guild_channel(&channel, path.into_inner(), claims.id)
        .await
        .and_then(|channel_option| {
            channel_option
                .map(CreateChannelResponse::try_from)
                .transpose()
        }) {
        Ok(Some(response)) => HttpResponse::Created().json(response),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("Error creating guild channel: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/{guild_id}/channel/{channel_id}")]
pub async fn get_guild_channel(
    repository: web::Data<DatabaseConnection>,
    path: web::Path<(i32, i32)>,
    claims: web::ReqData<Claims>,
) -> HttpResponse {
    match repository
        .find_channel(path.1, path.0, claims.id)
        .await
        .and_then(|channel_option| {
            channel_option
                .map(CreateChannelResponse::try_from)
                .transpose()
        }) {
        Ok(Some(channel)) => HttpResponse::Ok().json(channel),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/{guild_id}/role")]
pub async fn create_guild_role(
    repository: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
    claims: web::ReqData<Claims>,
    role: web::Json<CreateRoleRequest>,
) -> HttpResponse {
    if role.guild_id != path.into_inner() {
        return HttpResponse::BadRequest().finish();
    }

    match repository.create_guild_role(&role, claims.id).await {
        Ok(Some(role)) => HttpResponse::Created().json(role),
        Ok(None) => HttpResponse::BadRequest().finish(),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
