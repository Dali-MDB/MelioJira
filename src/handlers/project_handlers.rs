use crate::dtos::project::{CreateProjectRequest, UpdateProjectRequest};
use crate::services::projects_service::{
    deleteProjectService, getAllUserProjectsService, getProjectMembersService, getProjectService, removeMemberService,
    updateProjectService,
};
use crate::state::AppState;
use crate::{models::user::User, services::projects_service::createProjectService};
use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse, delete, get, post, put, web};
use uuid::Uuid;

#[post("/projects")]
pub async fn createProject(
    req: HttpRequest,
    state: web::Data<AppState>,
    project: web::Json<CreateProjectRequest>,
) -> Result<HttpResponse, Error> {
    let extension = req.extensions_mut();
    let current_user = extension
        .get::<User>()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Unauthorized"))?;
    let response = createProjectService(&state.pool, project.into_inner(), current_user.id).await?;
    Ok(HttpResponse::Created().json(response))
}

#[get("/projects/{id}")]
pub async fn getProject(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let response = getProjectService(&state.pool, path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[get("/projects/user/{user_id}")]
pub async fn getAllUserProjects(
    state: web::Data<AppState>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let response = getAllUserProjectsService(&state.pool, user_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[put("/projects/{id}")]
pub async fn updateProject(
    req: HttpRequest,
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    project: web::Json<UpdateProjectRequest>,
) -> Result<HttpResponse, Error> {
    let extension = req.extensions_mut();
    let current_user = extension
        .get::<User>()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Unauthorized"))?;
    let response = updateProjectService(
        &state.pool,
        id.into_inner(),
        project.into_inner(),
        current_user.id,
    )
    .await?;
    Ok(HttpResponse::Ok().json(response))
}

#[delete("/projects/{id}")]
pub async fn deleteProject(
    req: HttpRequest,
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let extension = req.extensions_mut();
    let current_user = extension
        .get::<User>()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Unauthorized"))?;
    let response = deleteProjectService(&state.pool, id.into_inner(), current_user.id).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[delete("/projects/{project_id}/members/{member_id}")]
pub async fn removeMember(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, Error> {
    let extension = req.extensions_mut();
    let current_user = extension
        .get::<User>()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Unauthorized"))?;
    let (project_id, member_id) = path.into_inner();
    removeMemberService(&state.pool, project_id, member_id, current_user.id).await?;
    Ok(HttpResponse::Ok().finish())
}

#[get("/projects/{project_id}/members")]
pub async fn getProjectMembers(
    state: web::Data<AppState>,
    project_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let response = getProjectMembersService(&state.pool, project_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(response))
}