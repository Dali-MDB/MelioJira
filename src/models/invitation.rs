use sqlx::{FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow)]
pub struct Invitation {
    id : Uuid,
    project_id : Uuid,
    invited_user_id : Uuid,
    invited_by : Uuid,
    created_at : DateTime<Utc>,
}