use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::models::sprint::Sprint;

#[derive(Debug, Clone, Validate, Deserialize)]
pub struct CreateSprintRequest {
    #[validate(length(max = 100))]
    pub name: String,
    #[validate(length(max = 10000))]
    pub goal: Option<String>,

    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Validate, Deserialize)]
pub struct UpdateSprintRequest {
    #[validate(length(max = 100))]
    pub title: Option<String>,
    #[validate(length(max = 10000))]
    pub description: Option<String>,

    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SprintResponse {
    pub id: Uuid,
    pub project_id: Uuid,
    pub created_by: Uuid,
    pub name: String,
    pub goal: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

impl From<Sprint> for SprintResponse {
    fn from(sprint: Sprint) -> Self {
        Self {
            id: sprint.id,
            project_id: sprint.project_id,
            created_by: sprint.created_by,
            name: sprint.name,
            goal: sprint.goal,
            start_date: sprint.start_date,
            end_date: sprint.end_date,
        }
    }
}
