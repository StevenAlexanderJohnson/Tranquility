use std::collections::BTreeMap;

use actix_web::{get, post, web, HttpResponse};
use data_access::{Guild, GuildRepository};

// use crate::data_access::guild_repository::GuildRepository;

// #[get("/")]
// pub async fn get_guilds(
//     claims: web::ReqData<BTreeMap<String, String>>,
//     repository: web::Data<GuildRepository>,
// ) -> HttpResponse {
//     let id = match claims.get("id") {
//         Some(id) => id,
//         None => return HttpResponse::Unauthorized().finish(),
//     };
//     match repository.find_member_guilds(id).await {
//         Ok(guilds) => HttpResponse::Ok().json(guilds),
//         Err(e) => {
//             println!("Failed to get guilds: {}", e);
//             HttpResponse::InternalServerError().finish()
//         }
//     }
// }

#[get("/{guild_id}")]
pub async fn get_guild(
    repository: web::Data<GuildRepository>,
    path: web::Path<i32>,
) -> HttpResponse {
    match repository.find_by_id(path.into_inner()).await {
        Ok(Some(guild)) => HttpResponse::Ok().json(guild),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => {
            println!("Failed to get guild: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// #[post("/{guild_id}/channel")]
// pub async fn create_channel(
//     repository: web::Data<GuildRepository>,
//     path: web::Path<String>,
//     body: web::Json<Channel>,
// ) -> HttpResponse {
//     match repository.add_channel(&path, body.into_inner()).await {
//         Ok(channel) => HttpResponse::Created().json(channel),
//         Err(e) => {
//             println!("{:?}", e);
//             HttpResponse::InternalServerError().finish()
//         }
//     }
// }

#[post("/")]
pub async fn create_guild(
    repository: web::Data<GuildRepository>,
    mut guild: web::Json<Guild>,
    claims: web::ReqData<BTreeMap<String, String>>,
) -> HttpResponse {
    if claims.get("id").is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    guild.created_date = Some(chrono::Utc::now());
    guild.updated_date = Some(chrono::Utc::now());

    match repository.insert(&guild).await {
        Ok(guild) => HttpResponse::Ok().json(guild),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
