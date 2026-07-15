use sqlx::{FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct Project{
    pub id : Uuid,
    pub owner_id : Uuid,
    pub title : String,
    pub description: String,
    pub creation_date : DateTime<Utc>,
}