use sqlx::{FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow)]
pub struct Project{
    pub id : Uuid,
    pub owner_id : Uuid,
    pub title : String,
    pub description: Option<String>,
    pub creation_date : DateTime<Utc>,
}