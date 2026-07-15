use serde::{Serialize, Deserialize};
use validator::Validate;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Validate, Deserialize)]
pub struct CreateProjectRequest{
    #[validate(length(max=100))]
    pub title : String,
    #[validate(length(max=255))]
    pub description: Option<String>,
}




#[derive(Debug, Validate, Deserialize)]
pub struct UpdateProjectRequest{
    #[validate(length(max=100))]
    pub title : Option<String>,
    #[validate(length(max=255))]
    pub description: Option<String>,
}



#[derive(Debug, Serialize)]
pub struct ProjectResponse{
    pub id : Uuid,
    pub owner_id : Uuid,
    pub title : String,
    pub description: Option<String>,
    pub creation_date : DateTime<Utc>,
}
