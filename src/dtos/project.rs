use crate::models::project::Project;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Validate, Deserialize)]
pub struct CreateProjectRequest {
    #[validate(length(max = 100))]
    pub title: String,
    #[validate(length(max = 10000))]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Validate, Deserialize)]
pub struct UpdateProjectRequest {
    #[validate(length(max = 100))]
    pub title: Option<String>,
    #[validate(length(max = 10000))]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectResponse {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub creation_date: DateTime<Utc>,
}

impl From<Project> for ProjectResponse {
    fn from(project: Project) -> Self {
        Self {
            id: project.id,
            title: project.title,
            description: project.description,
            owner_id: project.owner_id,
            creation_date: project.creation_date,
        }
    }
}
