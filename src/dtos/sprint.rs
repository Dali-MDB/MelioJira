use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct CreateSprintRequest {
    #[validate(length(max = 100))]
    pub name: String,
    #[validate(length(max = 10000))]
    pub goal: Option<String>,

    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateSprintRequest {
    #[validate(length(max = 100))]
    pub title: Option<String>,
    #[validate(length(max = 10000))]
    pub description: Option<String>,

    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct SprintResponse {
    pub id: Uuid,
    pub project_id: Uuid,
    pub created_by: Uuid,
    pub name: String,
    pub goal: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}
