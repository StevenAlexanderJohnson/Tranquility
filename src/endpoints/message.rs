use actix_web::{get, web, HttpResponse};
use data_access::DatabaseConnection;
use log::error;

use crate::jwt_handler::Claims;

#[get("/{guild_id}/channel/{channel_id}/message/page/{page_number}")]
pub async fn get_channel_messages(
    claims: web::ReqData<Claims>,
    repository: web::Data<DatabaseConnection>,
    path: web::Path<(i32, i32, i32)>,
) -> HttpResponse {
    match repository
        .get_channel_message(path.0, path.1, claims.id, path.2)
        .await
    {
        Ok(Some(message)) => HttpResponse::Ok().json(message),
        Ok(None) => HttpResponse::Unauthorized().finish(),
        Err(e) => {
            error!("Error while collecting channel messages. {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
