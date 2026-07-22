use crate::dtos::sprint::{self, CreateSprintRequest, SprintResponse, UpdateSprintRequest};
use crate::models::project::Project;
use crate::models::sprint::Sprint;
use crate::repositories::sprintRepository::{create_sprint, delete_sprint, get_sprint_by_id, get_sprints_by_project_id, update_sprint};
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


pub async fn updateSprintService(
    pool: &MySqlPool,
    sprint_id: Uuid,
    sprint: UpdateSprintRequest,
    user_id: Uuid
) -> Result<SprintResponse, Error> {
    //get sprint
    let current_sprint = get_sprint_by_id(pool, sprint_id).await.map_err(actix_web::error::ErrorInternalServerError)?;
   
    let name = sprint.name.unwrap_or(current_sprint.name);
    let goal = sprint.goal.unwrap_or(current_sprint.goal.unwrap_or("".to_string()));
    let start_date = sprint.start_date.unwrap_or(current_sprint.start_date);
    let end_date = sprint.end_date.unwrap_or(current_sprint.end_date);
    update_sprint(pool, sprint_id, &name, &goal, start_date, end_date).await.map_err(actix_web::error::ErrorInternalServerError)?;

    let updated = Sprint{
        id: current_sprint.id,
        project_id: current_sprint.project_id,
        created_by: current_sprint.created_by,
        name: name,
        goal: Some(goal),
        start_date: start_date,
        end_date: end_date,
      
    };
    let response = SprintResponse::from(updated);
    Ok(response)
}

pub async fn deleteSprintService(
    pool: &MySqlPool,
    sprint_id: Uuid,
    user_id: Uuid
) -> Result<(), Error> {
    delete_sprint(pool, sprint_id).await.map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(())
}