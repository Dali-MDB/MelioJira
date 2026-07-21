use actix_web::{HttpRequest, HttpResponse, Responder, Error, get, post, web};

use crate::{
    dtos::user::{CreateUserRequest, UserLoginRequest},
    repositories::userRepository::exists_by_email,
    state::AppState,
};

use crate::services::auth_services::{
    registerService, loginService
};


#[post("/register")]
pub async fn register(
    state: web::Data<AppState>,
    user: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, Error> {
    let response = registerService(
        &state.pool,
        user.into_inner(),
    )
    .await?;
    Ok(HttpResponse::Created().json(response))
}

#[post("/login")]
pub async fn login(
    state: web::Data<AppState>,
    cred: web::Json<UserLoginRequest>,
) -> Result<HttpResponse, Error>  {
    let response = loginService( state.get_ref(), &state.pool, cred.into_inner())
    .await?;
    Ok(HttpResponse::Created().json(response))
}

#[get("/me")]
pub async fn me(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("")
}
