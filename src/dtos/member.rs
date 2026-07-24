use crate::enums::user_role::Role;
use crate::models::member::Member;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct MemberResponse {
    pub user_id: Uuid,
    pub project_id: Uuid,
    pub role: Role,
    pub joined_at: DateTime<Utc>,
}

impl From<Member> for MemberResponse {
    fn from(member: Member) -> Self {
        Self {
            user_id: member.user_id,
            project_id: member.project_id,
            role: member.role,
            joined_at: member.joined_at,
        }
    }
}
