use actix_web::{
    HttpResponse, Responder,
    web::{ServiceConfig, scope},
};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/project"));
}
