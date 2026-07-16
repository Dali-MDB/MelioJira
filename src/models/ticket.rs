use crate::enums::ticket_priority::TicketPriority;
use crate::enums::ticket_status::TicketStatus;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Ticket {
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
