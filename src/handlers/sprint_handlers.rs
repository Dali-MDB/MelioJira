use crate::dtos::project::{CreateProjectRequest, UpdateProjectRequest};
use crate::dtos::sprint::{CreateSprintRequest, SprintResponse, UpdateSprintRequest};
use crate::services::sprints_service::{
    createSprintService, getSprintService, getProjectSprintsService, updateSprintService, deleteSprintService
};
use crate::state::AppState;
use crate::models::user::User;
use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse, delete, get, post, put, web};
use uuid::Uuid;


#[post("/sprints/{project_id}")]
pub async fn createSprint(
    req: HttpRequest, 
    state: web::Data<AppState>,
    sprint: web::Json<CreateSprintRequest>,
    project_id : web::Path<Uuid>
)-> Result<HttpResponse, Error> {
    let extensions = req.extensions_mut();
    let current_user = extensions.get::<User>()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Unauthorized"))?;

    let sprint  = createSprintService(&state.pool, sprint.into_inner(), project_id.into_inner(), current_user.id).await?;
    Ok(HttpResponse::Created().json(sprint))
}

#[get("/sprints/{project_id}")]
pub async fn getAllProjectSprints(
    req: HttpRequest,
    state: web::Data<AppState>,
    project_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let response = getProjectSprintsService(&state.pool, project_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(response))
}


#[get("/sprints/{project_id}/{sprint_id}")]
pub async fn getSprint(
    req: HttpRequest,
    state: web::Data<AppState>,
    project_id: web::Path<Uuid>,
    sprint_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let response = getSprintService(&state.pool, project_id.into_inner(), sprint_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[put("/sprints/{project_id}/{sprint_id}")]
pub async fn updateSprint(
    req: HttpRequest,
    state: web::Data<AppState>,
    project_id: web::Path<Uuid>,
    sprint_id: web::Path<Uuid>,
    sprint: web::Json<UpdateSprintRequest>,
) -> Result<HttpResponse, Error> {
    let response = updateSprintService(&state.pool, project_id.into_inner(), sprint_id.into_inner(), sprint.into_inner()).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[delete("/sprints/{project_id}/{sprint_id}")]
pub async fn deleteSprint(
    req: HttpRequest,
    state: web::Data<AppState>,
    project_id: web::Path<Uuid>,
    sprint_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let response = deleteSprintService(&state.pool, sprint_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(response))
}