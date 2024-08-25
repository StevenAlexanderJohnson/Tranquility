use sqlx::{Pool, Postgres};

use crate::AuthUser;

pub struct AuthRepository {
    pool: Pool<Postgres>,
}

impl AuthRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn insert(
        &self,
        auth_user: &AuthUser,
    ) -> Result<AuthUser, Box<dyn std::error::Error>> {
        sqlx::query_as::<_, AuthUser>(
            "INSERT INTO auth (username, password, email) VALUES ($1, $2, $3) RETURNING id, username, email;"
        )
        .bind(auth_user.username.to_string())
        .bind(auth_user.password.to_string())
        .bind(auth_user.email.as_ref().expect("Email has not been provided.").to_string())
        .fetch_one(&self.pool).await.map_err(|e| e.into())
    }

    pub async fn find(
        &self,
        username: String,
        password: String,
    ) -> Result<Option<AuthUser>, Box<dyn std::error::Error>> {
        match sqlx::query_as::<_, AuthUser>(
            "SELECT id, username, password, email WHERE (username = $1 or email = $1) and password = $2;"
        )
        .bind(username)
        .bind(password)
        .fetch_one(&self.pool).await {
            Ok(user) => Ok(Some(user)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e.into())
        }
    }
}
