use sqlx::{FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::enums::ticket_status::TicketStatus;
use crate::enums::ticket_priority::TicketPriority;

#[derive(Debug, FromRow)]
pub struct Ticket{
    pub id : Uuid,
    pub project_id : Uuid,
    pub sprint_id : Option<Uuid>,
    pub title : String,
    pub description : Option<String>,
    pub created_by : Uuid,
    pub assigned_to : Option<Uuid>,
    pub created_at : DateTime<Utc>,
    pub status : TicketStatus,
    pub priority: TicketPriority,
}