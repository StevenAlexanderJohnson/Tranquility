use mongodb::{bson::{doc, Bson}, Collection};
use server::auth_user::AuthUser;

#[derive(Clone)]
pub struct AuthRepository {
    collection: Collection<AuthUser>,
}

impl AuthRepository {
    pub fn new(collection: Collection<AuthUser>) -> Self {
        Self { collection }
    }

    pub async fn insert(&self, auth_user: AuthUser) -> Result<Bson, Box<dyn std::error::Error>> {
        let result = self.collection.insert_one(auth_user).await?;
        Ok(result.inserted_id)
    }

    pub async fn find(
        &self,
        auth_user: AuthUser,
    ) -> Result<Option<AuthUser>, Box<dyn std::error::Error>> {
        let guild = self
            .collection
            .find_one(doc! { "$or": [
                doc! {"email": auth_user.email.unwrap_or_default()},
                doc! {"username": auth_user.username}
                ],
                "password": auth_user.password
            })
            .await?;
        Ok(guild)
    }
}
