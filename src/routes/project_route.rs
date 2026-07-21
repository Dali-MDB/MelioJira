use actix_web::{
    HttpResponse, Responder,
    web::{ServiceConfig, scope},
};

use crate::handlers::project_handlers::{createProject, deleteProject, getAllUserProjects, getProject, updateProject};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/project"))
        .service(createProject)
        .service(getProject)
        .service(getAllUserProjects)
        .service(updateProject)
        .service(deleteProject);
    
}
