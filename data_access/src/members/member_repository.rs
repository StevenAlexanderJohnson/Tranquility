use sqlx::{Postgres, Transaction};

use crate::Member;

#[derive(Clone)]
pub struct MemberRepository {}

impl MemberRepository {
    pub async fn add_user_to_guild(
        &self,
        member_id: i32,
        guild_id: i32,
        adder_id: i32,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<Member, Box<dyn std::error::Error>> {
        sqlx::query_as::<_, Member>("INSERT INTO member (user_id, guild_id, user_who_added) VALUES ($1, $2, $3) RETURNING id, user_id, guild_id, user_who_added, created_date, updated_date;")
        .bind(member_id)
        .bind(guild_id)
        .bind(adder_id)
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| e.into())
    }
}
