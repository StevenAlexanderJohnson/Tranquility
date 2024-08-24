use std::collections::{BTreeMap, HashSet};

use actix_web::{get, post, web, HttpResponse};
use mongodb::bson::oid::ObjectId;
use server::{
    channel::Channel,
    guild::Guild,
    member::ServerMember,
    role::{Intent, Role},
};

use crate::data_access::guild_repository::GuildRepository;

#[get("/")]
pub async fn get_guilds(
    claims: web::ReqData<BTreeMap<String, String>>,
    repository: web::Data<GuildRepository>,
) -> HttpResponse {
    let id = match claims.get("id") {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };
    match repository.find_member_guilds(id).await {
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
) -> HttpResponse {
    match repository.find(&path).await {
        Ok(Some(guild)) => HttpResponse::Ok().json(guild),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => {
            println!("Failed to get guild: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/{guild_id}/channel")]
pub async fn create_channel(
    repository: web::Data<GuildRepository>,
    path: web::Path<String>,
    body: web::Json<Channel>,
) -> HttpResponse {
    match repository.add_channel(&path, body.into_inner()).await {
        Ok(channel) => HttpResponse::Created().json(channel),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/")]
pub async fn create_guild(
    repository: web::Data<GuildRepository>,
    mut guild: web::Json<Guild>,
    claims: web::ReqData<BTreeMap<String, String>>,
) -> HttpResponse {
    if claims.get("id").is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    guild.members = vec![ServerMember {
        joined_date: chrono::Utc::now(),
        member: match ObjectId::parse_str(claims.get("id").unwrap()) {
            Ok(id) => id,
            Err(_) => return HttpResponse::Unauthorized().finish(),
        },
        roles: HashSet::from([Role {
            id: None,
            name: "Admin".to_string(),
            intents: vec![
                Intent::GuildBanAdd,
                Intent::GuildBanRemove,
                Intent::GuildDelete,
                Intent::GuildMemberAdd,
                Intent::GuildMemberRemove,
                Intent::GuildMemberUpdate,
                Intent::GuildRoleCreate,
                Intent::GuildRoleDelete,
                Intent::GuildRoleUpdate,
            ],
        }]),
    }];

    guild.created_at = Some(chrono::Utc::now());
    guild.updated_at = Some(chrono::Utc::now());

    match repository.insert(guild.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to create guild: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
