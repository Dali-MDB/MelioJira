use sqlx::{FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::enums::ticket_status::TicketStatus;
use crate::enums::ticket_priority::TicketPriority;

pub struct Ticket{
    id : Uuid,
    project_id : Uuid,
    sprint_id : Option<Uuid>,
    title : String,
    description : Option<String>,
    created_by : Uuid,
    assigned_to : Option<Uuid>,
    created_at : DateTime<Utc>,
    status : TicketStatus,
    priority: TicketPriority,
}