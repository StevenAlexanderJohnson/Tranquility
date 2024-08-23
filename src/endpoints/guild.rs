use std::collections::BTreeMap;

use actix_web::{get, post, web, HttpResponse};
use server::guild::Guild;

use crate::data_access::guild_repository::GuildRepository;

#[get("/")]
pub async fn get_guilds(repository: web::Data<GuildRepository>) -> HttpResponse {
    match repository.find_member_guilds("1234").await {
        Ok(guilds) => HttpResponse::Ok().json(guilds),
        Err(e) => {
            println!("Failed to get guilds: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/{guild_id}")]
pub async fn get_guild(
    repository: web::Data<GuildRepository>,
    path: web::Path<String>,
    claims: web::ReqData<BTreeMap<String, String>>
) -> HttpResponse {
    println!("Claims: {:?}", claims.contains_key("sub"));
    match repository.find(&path).await {
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
    repository: web::Data<GuildRepository>,
    guild: web::Json<Guild>,
) -> HttpResponse {
    match repository.insert(guild.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to create guild: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
