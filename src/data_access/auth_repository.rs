use mongodb::{bson::doc, Collection};
use server::auth_user::AuthUser;

#[derive(Clone)]
pub struct AuthRepository {
    collection: Collection<AuthUser>,
}

impl AuthRepository {
    pub fn new(collection: Collection<AuthUser>) -> Self {
        Self { collection }
    }

    pub async fn insert(&self, auth_user: AuthUser) -> Result<(), Box<dyn std::error::Error>> {
        let _ = self.collection.insert_one(auth_user).await?;
        Ok(())
    }

    pub async fn find(
        &self,
        auth_user: AuthUser,
    ) -> Result<Option<AuthUser>, Box<dyn std::error::Error>> {
        let guild = self
            .collection
            .find_one(doc! {"$or": {"email": auth_user.email, "username": auth_user.username}})
            .await?;
        Ok(guild)
    }
}
