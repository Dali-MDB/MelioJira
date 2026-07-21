use actix_web::{
    HttpResponse, Responder,
    web::{ServiceConfig, scope},
};

use crate::handlers::auth_handlers::{register, login};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/auth")
        .service(register)
        .service(login)
    );
}
