use std::collections::BTreeMap;

use actix_web::{get, post, web, HttpResponse};
use data_access::{DatabaseConnection, Guild};

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
        Ok(guilds) => HttpResponse::Ok().json(guilds),
        Err(e) => {
            println!("{e:?}");
            HttpResponse::InternalServerError().finish()
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
