use crate::enums::invitation_status::InvitationStatus;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Invitation {
    pub id: Uuid,
    pub project_id: Uuid,
    pub invited_user_id: Uuid,
    pub invited_by: Uuid,
    pub status: InvitationStatus,
    pub created_at: DateTime<Utc>,
}
