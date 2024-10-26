use sqlx::{Postgres, Transaction};

use crate::Member;

use data_models::CreateMemberRequest;

#[derive(Clone)]
pub struct MemberRepository {}

impl MemberRepository {
    pub async fn add_user_to_guild(
        &self,
        member_request: &CreateMemberRequest,
        adder_id: i32,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<Member, Box<dyn std::error::Error>> {
        sqlx::query_as::<_, Member>("INSERT INTO member (user_id, guild_id, user_who_added) VALUES ($1, $2, $3) RETURNING id, user_id, guild_id, user_who_added, created_date, updated_date;")
        .bind(member_request.user_id)
        .bind(member_request.guild_id)
        .bind(adder_id)
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| e.into())
    }
}
