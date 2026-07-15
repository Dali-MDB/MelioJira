use sqlx::{FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct Comment{
    id : Uuid,
    by : Uuid,
    ticket_id : Uuid,
    content : String,
    time_stamp : DateTime<Utc>,
    
}