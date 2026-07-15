use sqlx::{FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::enums::user_role::Role;

#[derive(Debug, FromRow)]
pub struct Member{
    pub user_id : Uuid,
    pub project_id : Uuid,
    pub role : Role,
    pub joined_at : DateTime<Utc>,
}