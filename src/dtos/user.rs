use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Validate, Deserialize)]
pub struct CreateUserRequest {
    pub user_name: String,

    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub user_name: String,
    pub email: String,
}

#[derive(Debug, Clone, Validate, Deserialize)]
pub struct UserLoginRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}
