use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Comment {
    pub id: Uuid,
    pub by: Uuid,
    pub ticket_id: Uuid,
    pub content: String,
    pub time_stamp: DateTime<Utc>,
}
