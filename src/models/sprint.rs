use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Sprint {
    pub id: Uuid,
    pub project_id: Uuid,
    pub created_by: Uuid,
    pub name: String,
    pub goal: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}
