use crate::auth::password::hash_password;
use crate::dtos::user::{CreateUserRequest, UserLoginRequest, UserResponse};
use crate::repositories::userRepository::{
    exists_by_email, fetch_user_by_email, fetch_user_by_id, insert_user,
};
use actix_web::{Error, error::ErrorConflict};
use sqlx::MySqlPool;

pub async fn registerService(
    pool: &MySqlPool,
    user: CreateUserRequest,
) -> Result<UserResponse, Error> {
    //check if this email is already registered
    let exists = exists_by_email(pool, &user.email).await?;

    if exists {
        return Err(ErrorConflict("Email already exists"));
    }

    //hash the password
    let password_hash =
        hash_password(&user.password).map_err(actix_web::error::ErrorInternalServerError)?;
    //insert the user
    let user_db = insert_user(pool, &user.user_name, &user.email, &password_hash)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let response = UserResponse {
        id: user_db.id,
        user_name: user_db.user_name,
        email: user_db.email,
    };

    Ok(response)
}


