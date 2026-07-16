use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct CreateUserRequest {
    pub user_name: String,

    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub user_name: String,
    pub email: String,
}
