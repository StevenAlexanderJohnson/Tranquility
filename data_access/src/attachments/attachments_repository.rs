use sqlx::{Postgres, Transaction};

use crate::{Attachment, AttachmentMapping};

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
        .await
        {
            Ok(x) => Ok(Some(Attachment {
                file_path: None,
                ..x
            })),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn create_message_attachment_mapping(
        &self,
        mapping: &AttachmentMapping,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match sqlx::query(
            r#"
            INSERT INTO attachment_mapping (post_id, attachment_id)
            VALUES ($1, $2);
            "#,
        )
        .bind(&mapping.post_id)
        .bind(&mapping.attachment_id)
        .execute(&mut **tx)
        .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}
