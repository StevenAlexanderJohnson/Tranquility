use mongodb::Collection;
use server::member::Member;

#[derive(Clone)]
pub struct MemberRepository {
    collection: Collection<Member>
}

impl MemberRepository {
    pub fn new(collection: Collection<Member>) -> Self {
        Self { collection }
    }

    pub async fn insert(&self, member: Member) -> Result<(), Box<dyn std::error::Error>> {
        let _ = self.collection.insert_one(member).await?;
        Ok(())
    }
}