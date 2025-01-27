use sqlx::{Postgres, Transaction};

use crate::{Intent, Role};

#[derive(Clone)]
pub struct RoleRepository {}

impl RoleRepository {
    pub async fn create_role(
        &self,
        guild_id: i32,
        name: &str,
        user_id: &i32,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<Role, Box<dyn std::error::Error>> {
        sqlx::query_as::<_, Role>(
            r#"
            INSERT INTO role (guild_id, name) 
            SELECT $1, $2 WHERE EXISTS (SELECT 1 FROM guild WHERE owner_id = $3)
            RETURNING id, name, guild_id, created_date, updated_date;
            "#,
        )
        .bind(guild_id)
        .bind(name)
        .bind(user_id)
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| e.into())
    }

    pub async fn add_row_intent(
        &self,
        role_id: i32,
        intent: i32,
        user_id: &i32,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<Intent, Box<dyn std::error::Error>> {
        sqlx::query_as::<_, Intent>(
            r#"
            INSERT INTO role_intent (role_id, value)
            SELECT $1, $2 WHERE EXISTS (SELECT 1 FROM guild WHERE owner_id = $3)
            RETURNING id, role_id, value, created_date, updated_date;"#,
        )
        .bind(role_id)
        .bind(intent)
        .bind(user_id)
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| e.into())
    }
}
