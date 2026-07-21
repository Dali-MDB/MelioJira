use crate::auth::jwt::{Claim, JWTResponse, generate_token};
use crate::auth::password::{hash_password, verify_password};
use crate::dtos::user::{CreateUserRequest, UserLoginRequest, UserResponse};
use crate::repositories::userRepository::{
    exists_by_email, fetch_user_by_email, fetch_user_by_id, insert_user,
};
use crate::state::AppState;
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::{Error, error::ErrorConflict};
use chrono::{Duration, Utc};
use sqlx::MySqlPool;

pub async fn registerService(
    pool: &MySqlPool,
    user: CreateUserRequest,
) -> Result<UserResponse, Error> {
    //check if this email is already registered
    let exists = exists_by_email(pool, &user.email)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

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

pub async fn loginService(
    state: &AppState,
    pool: &MySqlPool,
    cred: UserLoginRequest,
) -> Result<JWTResponse, actix_web::Error> {
    //fetch user and raise error if doesn't exist
    let user = fetch_user_by_email(pool, &cred.email)
        .await
        .map_err(ErrorUnauthorized)?;
    //verify password
    let rslt = verify_password(&cred.password, &user.password).map_err(ErrorUnauthorized)?;
    //result is always true, cuz otherwise it raises an error
    //user verified, hence we generate access token and refresh token
    let now = Utc::now();

    let payload = Claim {
        sub: user.id,
        iat: now.timestamp() as usize,
        exp: (now + Duration::minutes(30)).timestamp() as usize,
    };
    let token = generate_token(&payload, &state.config.jwt_secret).map_err(ErrorInternalServerError)?;
    let response = JWTResponse{
        access_token: token,
    };
    Ok(response)
}
