use actix_web::{HttpRequest, HttpResponse, Responder, body, get, post, web};

use crate::{
    dtos::user::{CreateUserRequest, UserLoginRequest},
    repositories::userRepository::exists_by_email,
    state::AppState,
};

#[post("/register")]
pub async fn register(
    state: web::Data<AppState>,
    user: web::Json<CreateUserRequest>,
) -> impl Responder {
    HttpResponse::Ok().body("")
}

#[post("/login")]
pub async fn login(
    state: web::Data<AppState>,
    cred: web::Json<UserLoginRequest>,
) -> impl Responder {
    HttpResponse::Ok().body("")
}

#[get("/me")]
pub async fn me(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("")
}
