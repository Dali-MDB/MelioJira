use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct CreateProjectRequest {
    #[validate(length(max = 100))]
    pub title: String,
    #[validate(length(max = 10000))]
    pub description: Option<String>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateProjectRequest {
    #[validate(length(max = 100))]
    pub title: Option<String>,
    #[validate(length(max = 10000))]
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProjectResponse {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub creation_date: DateTime<Utc>,
}
