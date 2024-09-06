use sqlx::{Postgres, Transaction};

use crate::{Role, Intent};

#[derive(Clone)]
pub struct RoleRepository{}

impl RoleRepository {
    pub async fn create_role(
        &self,
        guild_id: i32,
        name: &str,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<Role, Box<dyn std::error::Error>> {
        sqlx::query_as::<_, Role>("INSERT INTO role (guild_id, name) VALUES ($1, $2) RETURNING id, name, guild_id, created_id, updated_id;")
            .bind(guild_id)
            .bind(name)
            .fetch_one(&mut **tx)
            .await
            .map_err(|e| e.into())
    }

    pub async fn add_row_intent(
        &self,
        role_id: i32,
        intent: i32,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<Intent, Box<dyn std::error::Error>> {
        sqlx::query_as::<_, Intent>("INSERT INTO role_intent (role_id, value) VALUES ($1, $2) RETURNING id, role_id, value, created_date, updated_date;")
            .bind(role_id)
            .bind(intent)
            .fetch_one(&mut **tx)
            .await
            .map_err(|e| e.into())
    }
}