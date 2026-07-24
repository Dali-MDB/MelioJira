use crate::enums::invitation_status::InvitationStatus;
use crate::models::invitation::Invitation;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateInvitationRequest {
    pub invited_user_id: Uuid,
}

#[derive(Debug, Clone, Serialize)]
pub struct InvitationResponse {
    pub id: Uuid,
    pub project_id: Uuid,
    pub invited_user_id: Uuid,
    pub invited_by: Uuid,
    pub status: InvitationStatus,
    pub created_at: DateTime<Utc>,
}

impl From<Invitation> for InvitationResponse {
    fn from(invitation: Invitation) -> Self {
        Self {
            id: invitation.id,
            project_id: invitation.project_id,
            invited_user_id: invitation.invited_user_id,
            invited_by: invitation.invited_by,
            status: invitation.status,
            created_at: invitation.created_at,
        }
    }
}
