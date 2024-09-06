use std::collections::BTreeMap;

use actix_web::{get, post, web, HttpResponse, ResponseError};
use data_access::{Channel, DatabaseConnection, Guild, RoleRequest};

#[get("/")]
pub async fn get_guilds(
    claims: web::ReqData<BTreeMap<String, String>>,
    repository: web::Data<DatabaseConnection>,
) -> HttpResponse {
    let id = match claims.get("id").and_then(|id| id.parse::<i32>().ok()) {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    match repository.find_joined_guild(id).await {
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
    claims: web::ReqData<BTreeMap<String, String>>,
    repository: web::Data<DatabaseConnection>,
) -> HttpResponse {
    let id = match claims.get("id").and_then(|id| id.parse::<i32>().ok()) {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    match repository.find_owned_guilds(id).await {
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
    claims: web::ReqData<BTreeMap<String, String>>,
    path: web::Path<i32>,
) -> HttpResponse {
    let id = match claims.get("id").and_then(|id| id.parse::<i32>().ok()) {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    match repository.find_guild_by_id(path.into_inner(), id).await {
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
    mut guild: web::Json<Guild>,
    claims: web::ReqData<BTreeMap<String, String>>,
) -> HttpResponse {
    let id = match claims.get("id").and_then(|id| id.parse::<i32>().ok()) {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    guild.owner_id = Some(id);

    match repository.create_guild(&guild).await {
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
    claims: web::ReqData<BTreeMap<String, String>>,
) -> HttpResponse {
    let id = match claims.get("id").and_then(|id| id.parse::<i32>().ok()) {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    match repository.find_guild_channels(path.into_inner(), id).await {
        Ok(Some(guilds)) => HttpResponse::Ok().json(guilds),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => {
            println!("Failed to get guild channels: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/{guild_id}/channel")]
pub async fn create_guild_channel(
    repository: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
    claims: web::ReqData<BTreeMap<String, String>>,
    mut channel: web::Json<Channel>,
) -> HttpResponse {
    let id = match claims.get("id").and_then(|id| id.parse::<i32>().ok()) {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    channel.guild_id = Some(path.into_inner());

    match repository.create_guild_channel(&channel, id).await {
        Ok(Some(channel)) => HttpResponse::Created().json(channel),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/{guild_id}/channel/{channel_id}")]
pub async fn get_guild_channel(
    repository: web::Data<DatabaseConnection>,
    path: web::Path<(i32, i32)>,
    claims: web::ReqData<BTreeMap<String, String>>,
) -> HttpResponse {
    let id = match claims.get("id").and_then(|id| id.parse::<i32>().ok()) {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    match repository.find_channel(path.1, path.0, id).await {
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
    claims: web::ReqData<BTreeMap<String, String>>,
    role: web::Json<RoleRequest>,
) -> HttpResponse {
    let id = match claims.get("id").and_then(|id| id.parse::<i32>().ok()) {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    if role.guild_id != path.into_inner() {
        return HttpResponse::BadRequest().finish();
    }

    match repository.create_guild_role(&role, id).await {
        Ok(Some(role)) => HttpResponse::Created().json(role),
        Ok(None) => HttpResponse::BadRequest().finish(),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
