use sqlx::{FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};


pub struct Sprint{
    pub id : Uuid,
    pub project_id : Uuid,
    pub name : String,
    pub goal : Option<String>,
    pub start_date : DateTime<Utc>,
    pub end_date : DateTime<Utc>,
}