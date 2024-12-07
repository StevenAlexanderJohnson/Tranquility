use sqlx::{Postgres, Transaction};

use crate::Attachment;

#[derive(Clone)]
pub struct AttachmentsRepository {}

impl AttachmentsRepository {
    pub async fn insert(
        &self,
        attachment: &Attachment,
        user_id: i32,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<Attachment>, Box<dyn std::error::Error>> {
        match sqlx::query_as::<_, Attachment>(
            r#"
            INSERT INTO attachment (file_name, file_path, file_size, mime_type, user_uploaded)
            SELECT $1, $2, $3, $4, $5
            RETURNING id, file_name, file_path, file_size, mime_type, created_date;
            "#,
        )
        .bind(&attachment.file_name)
        .bind(&attachment.file_path)
        .bind(&attachment.file_size)
        .bind(&attachment.mime_type)
        .bind(user_id)
        .fetch_one(&mut **tx)
        .await {
            Ok(x) => Ok(Some(x)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
