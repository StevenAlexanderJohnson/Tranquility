use std::collections::HashSet;

use crate::role::Role;

pub struct Member {
    pub id: uuid::Uuid,
    pub name: String,
    pub joined_date: chrono::DateTime<chrono::Utc>,
    pub friends: HashSet<uuid::Uuid>,
}

pub struct ServerMember {
    pub member: Member,
    pub roles: HashSet<Role>,
    pub joined_date: chrono::DateTime<chrono::Utc>,
}