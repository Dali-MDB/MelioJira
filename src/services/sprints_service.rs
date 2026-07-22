use crate::dtos::sprint::{self, CreateSprintRequest, SprintResponse};
use crate::models::project::Project;
use crate::repositories::sprintRepository::{create_sprint, get_sprints_by_project_id, get_sprint_by_id};
use crate::services::projects_service::getProjectService;
use actix_web::error::Error;
use sqlx::MySqlPool;
use uuid::Uuid;

pub async fn createSprintService(
    pool: &MySqlPool,
    sprint: CreateSprintRequest,
    project_id: Uuid,
    user_id: Uuid,
) -> Result<SprintResponse, Error> {
    //fetch project
    let project = getProjectService(pool, project_id).await?;
    if project.owner_id != user_id {
        return Err(actix_web::error::ErrorUnauthorized(
            "You are not the owner of this project".to_string(),
        ));
    }
    let goal = sprint.goal.unwrap_or("".to_string());
    //create sprint
    let sprint = create_sprint(
        pool,
        &sprint.name,
        &goal,
        sprint.start_date,
        sprint.end_date,
        user_id,
    )
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;
    let response = SprintResponse::from(sprint);
    Ok(response)
}


pub async fn getSprintService(
    pool: &MySqlPool,
    sprint_id: Uuid,
) -> Result<SprintResponse, Error> {
    let sprint = get_sprint_by_id(pool, sprint_id).await.map_err(actix_web::error::ErrorInternalServerError)?;
    let response = SprintResponse::from(sprint);
    Ok(response)
}

pub async fn getProjectSprintsService(
    pool: &MySqlPool,
    project_id: Uuid,
) -> Result<Vec<SprintResponse>, Error> {
    let sprints = get_sprints_by_project_id(pool, project_id).await.map_err(actix_web::error::ErrorInternalServerError)?;
    let response = sprints.into_iter().map(|sprint| SprintResponse::from(sprint)).collect().map_err(actix_web::error::ErrorInternalServerError);
    Ok(response)
}


