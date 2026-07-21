use crate::dtos::project::{CreateProjectRequest, ProjectResponse, UpdateProjectRequest};
use crate::models::project::Project;
use crate::repositories::projectRepository::{
    create_project, delete_project, get_all_user_projects, get_project_by_id, update_project,
};
use actix_web::error::Error;
use sqlx::MySqlPool;
use uuid::Uuid;

pub async fn createProjectService(
    pool: &MySqlPool,
    project: CreateProjectRequest,
    owner_id: Uuid,
) -> Result<ProjectResponse, Error> {
    let description = project.description.as_deref().unwrap_or("");
    let project = create_project(pool, &project.title, description, owner_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let response = ProjectResponse::from(project);
    Ok(response)
}

pub async fn getProjectService(pool: &MySqlPool, id: Uuid) -> Result<ProjectResponse, Error> {
    let project = get_project_by_id(pool, id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let response = ProjectResponse::from(project);
    Ok(response)
}

pub async fn getAllUserProjectsService(
    pool: &MySqlPool,
    user_id: Uuid,
) -> Result<Vec<ProjectResponse>, Error> {
    let projects = get_all_user_projects(pool, user_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let responses = projects
        .into_iter()
        .map(|project| ProjectResponse::from(project))
        .collect();
    Ok(responses)
}

pub async fn updateProjectService(
    pool: &MySqlPool,
    id: Uuid,
    project: UpdateProjectRequest,
    user_id: Uuid,
) -> Result<ProjectResponse, Error> {
    //get the current project
    let current_project = get_project_by_id(pool, id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    //check the owner is the current user
    if current_project.owner_id != user_id {
        return Err(actix_web::error::ErrorUnauthorized(
            "You are not the owner of this project".to_string(),
        ));
    }
    //unwrap project title and description (if not provided keep the old one)
    let title = project.title.as_deref().unwrap_or(&current_project.title);
    let current_description = current_project.description.unwrap_or("".to_string()); //defaulte to ""
    let description = project
        .description
        .as_deref()
        .unwrap_or(&current_description);

    //update the project
    update_project(pool, id, title, description)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    //reconstruct the response
    let updated = Project {
        id: current_project.id,
        title: title.to_string(),
        description: if description.is_empty() {
            None
        } else {
            Some(description.to_string())
        },
        owner_id: current_project.owner_id,
        creation_date: current_project.creation_date,
    };
    let response = ProjectResponse::from(updated);
    Ok(response)
}

pub async fn deleteProjectService(pool: &MySqlPool, id: Uuid, user_id: Uuid) -> Result<(), Error> {
    //check the owner is the current user
    let current_project = get_project_by_id(pool, id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    if current_project.owner_id != user_id {
        return Err(actix_web::error::ErrorUnauthorized(
            "You are not the owner of this project".to_string(),
        ));
    }
    //delete the project
    delete_project(pool, id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(())
}
