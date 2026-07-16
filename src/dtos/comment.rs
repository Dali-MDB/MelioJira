use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;


#[derive(Debug, Clone, Validate, Deserialize)]
pub struct CreateCommentRequest{
    #[validate(length(min=3, max=10000))]
    pub content : String,
}



#[derive(Debug, Clone, Validate, Deserialize)]
pub struct UpdateCommentRequest{
    #[validate(length(min=3, max=10000))]
    pub content : String,
}



#[derive(Debug, Clone, Deserialize)]
pub struct CommentResponse{
    pub id: Uuid,
    pub by: Uuid,
    pub ticket_id: Uuid,
    pub content: String,
    pub time_stamp: DateTime<Utc>,
}




