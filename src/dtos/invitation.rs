use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Deserialize)]
pub struct CreateInvitationRequest {
    pub invited_user_id: Uuid
}

#[derive(Debug, Serialize)]
pub struct InvitationResponse {
    id: Uuid,
    project_id: Uuid,
    invited_user_id: Uuid,
    invited_by: Uuid,
    created_at: DateTime<Utc>,
}