use crate::enums::user_role::Role;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Member {
    pub user_id: Uuid,
    pub project_id: Uuid,
    pub role: Role,
    pub joined_at: DateTime<Utc>,
}
