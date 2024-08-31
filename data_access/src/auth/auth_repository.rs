use sqlx::{Pool, Postgres, Transaction};

use crate::AuthUser;

#[derive(Clone)]
pub struct AuthRepository {}

impl<'a> AuthRepository {
    pub async fn insert(
        &self,
        auth_user: &AuthUser,
        pool: &Pool<Postgres>,
    ) -> Result<AuthUser, Box<dyn std::error::Error>> {
        sqlx::query_as::<_, AuthUser>(
            "INSERT INTO auth (username, password, email) VALUES ($1, $2, $3) RETURNING id, username, email, refresh_token;"
        )
        .bind(auth_user.username.to_string())
        .bind(auth_user.password.as_ref().expect("Password was not provided.").to_string())
        .bind(auth_user.email.as_ref().expect("Email has not been provided.").to_string())
        .fetch_one(pool)
        .await.map_err(|e| e.into())
    }

    pub async fn find(
        &self,
        auth_user: &AuthUser,
        tx: &mut Transaction<'_, Postgres>
    ) -> Result<Option<AuthUser>, Box<dyn std::error::Error>> {
        match sqlx::query_as::<_, AuthUser>(
            "UPDATE auth SET refresh_token = md5(random()::text) WHERE username = $1 AND password = $2 RETURNING id, username, email, refresh_token;"
        )
        .bind(&auth_user.username)
        .bind(&auth_user.password.as_ref().expect("password was not provided"))
        .fetch_one(&mut **tx).await {
            Ok(user) => Ok(Some(user)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e.into())
        }
    }

    pub async fn update_refresh_token(
        &self,
        user_id: i32,
        token: String,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<AuthUser>, Box<dyn std::error::Error>> {
        match sqlx::query_as::<_, AuthUser>(
            "UPDATE auth SET refresh_token = md5(random()::text) WHERE id = $1 AND refresh_token = $2 RETURNING id, username, email, refresh_token, updated_date;"
        )
        .bind(&user_id)
        .bind(&token)
        .fetch_one(&mut **tx).await {
            Ok(user) => Ok(Some(user)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e.into())
        }
    }
}
