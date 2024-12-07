use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{post, web, HttpResponse};
use data_access::{Attachment, DatabaseConnection};

use crate::{file_handler::LocalFileHandler, jwt_handler::Claims};

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "100MB")]
    file: TempFile,
}

#[post("")]
pub async fn upload_attachments(
    claims: web::ReqData<Claims>,
    file_handler: web::Data<LocalFileHandler>,
    repository: web::Data<DatabaseConnection>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> HttpResponse {
    let file_handler = file_handler.into_inner();

    let file_type = match &form.file.content_type {
        Some(x) => x,
        None => {
            return HttpResponse::BadRequest().body("Content Type was not set for the attachment.");
        }
    };

    let file_name = match &form.file.file_name {
        Some(name) => name.clone(),
        None => {
            return HttpResponse::BadRequest().body("File provided had no name");
        }
    };

    let (file_name, file_path) = match file_handler.store_file(&form.file, &file_name) {
        Ok(x) => x,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    match repository
        .create_attachment(
            &Attachment {
                id: None,
                file_name: Some(file_name),
                file_size: Some(form.file.size as i64),
                file_path: Some(file_path.clone()),
                mime_type: Some(file_type.essence_str().into()),
                created_date: None,
            },
            claims.id,
        )
        .await
    {
        Ok(Some(attachment)) => HttpResponse::Created().json(&attachment),
        Ok(None) => {
            if let Err(e) = file_handler.delete_file(&file_path) {
                println!("Failed to delete file after failed insert.: {:?}", e);
            };
            HttpResponse::InternalServerError()
                .body("Error occurred while saving your file to the database.")
        }
        Err(e) => {
            if let Err(e) = file_handler.delete_file(&file_path) {
                println!("Failed to delete file after failed insert.: {:?}", e);
            };
            println!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
