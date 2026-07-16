use crate::enums::ticket_priority::TicketPriority;
use crate::enums::ticket_status::TicketStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct CreateTicketRequest {
    #[validate(length(min = 3, max = 100))]
    pub title: String,
    #[validate(length(max = 10000))]
    pub description: Option<String>,
    pub priority: TicketPriority,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateTicketRequest {
    #[validate(length(min = 3, max = 100))]
    pub title: Option<String>,
    #[validate(length(max = 10000))]
    pub description: Option<String>,
    pub priority: Option<TicketPriority>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTicketStatusRequest {
    pub status: TicketStatus,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTicketAssigneeRequest {
    pub assignee_id: Option<Uuid>, //becaus when we un assign it we pass Null
}

#[derive(Debug, Deserialize)]
pub struct UpdateTicketSprintRequest {
    pub sprint_id: Option<Uuid>, //becuase when we take it back to backlog we pass Null
}

#[derive(Debug, Serialize)]
pub struct TicketResponse {
    pub id: Uuid,
    pub project_id: Uuid,
    pub sprint_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub creator_id: Uuid,
    pub assignee_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub status: TicketStatus,
    pub priority: TicketPriority,
}
