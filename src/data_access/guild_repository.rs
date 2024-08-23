use std::str::FromStr;

use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};
use server::guild::Guild;

#[derive(Clone)]
pub struct GuildRepository {
    collection: Collection<Guild>,
}

impl GuildRepository {
    pub fn new(collection: Collection<Guild>) -> Self {
        Self { collection }
    }

    pub async fn insert(&self, guild: Guild) -> Result<(), Box<dyn std::error::Error>> {
        let _ = self.collection.insert_one(guild).await?;
        Ok(())
    }

    pub async fn find(&self, id: &str) -> Result<Option<Guild>, Box<dyn std::error::Error>> {
        let object_id = ObjectId::from_str(id)?;
        let guild = self.collection.find_one(doc! {"_id": object_id}).await?;
        Ok(guild)
    }

    pub async fn find_member_guilds(
        &self,
        member_id: &str,
    ) -> Result<Vec<Guild>, Box<dyn std::error::Error>> {
        let object_id = ObjectId::from_str(member_id)?;
        let mut cursor = self
            .collection
            .find(doc! {"members._id": object_id})
            .await?;

        let mut output = vec![];

        while cursor.advance().await? {
            output.push(cursor.deserialize_current()?);
        }

        Ok(output)
    }
}
