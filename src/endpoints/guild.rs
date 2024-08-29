use std::collections::BTreeMap;

use actix_web::{get, post, web, HttpResponse};
use data_access::{Guild, GuildRepository, MemberRepository};

#[get("/")]
pub async fn get_guilds(
    claims: web::ReqData<BTreeMap<String, String>>,
    repository: web::Data<GuildRepository>,
) -> HttpResponse {
    let id = match claims.get("id").and_then(|id| id.parse::<i32>().ok()) {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    match repository.find_guilds(id).await {
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
    repository: web::Data<GuildRepository>,
) -> HttpResponse {
    let id = match claims.get("id").and_then(|id| id.parse::<i32>().ok()) {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    match repository.find_owner_guilds(id).await {
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
    claims: web::ReqData<BTreeMap<String, String>>,
    path: web::Path<i32>,
) -> HttpResponse {
    let id = match claims.get("id").and_then(|id| id.parse::<i32>().ok()) {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    match repository.find_by_id(path.into_inner(), id).await {
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
    member_repository: web::Data<MemberRepository>,
    mut guild: web::Json<Guild>,
    claims: web::ReqData<BTreeMap<String, String>>,
) -> HttpResponse {
    let id = match claims.get("id").and_then(|id| id.parse::<i32>().ok()) {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    guild.owner_id = Some(id);

    let new_guild = match repository.insert(&guild).await {
        Ok(guild) => guild,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    match member_repository
        .add_user_to_guild(id, new_guild.id.unwrap(), id)
        .await
    {
        Ok(_) => HttpResponse::Ok().json(guild),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
