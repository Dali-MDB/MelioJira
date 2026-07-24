use crate::dtos::ticket::{
    CreateTicketRequest, TicketResponse, UpdateTicketAssigneeRequest, UpdateTicketRequest,
    UpdateTicketSprintRequest, UpdateTicketStatusRequest,
};
use crate::enums::ticket_status::TicketStatus;
use crate::models::project::Project;
use crate::models::ticket::Ticket;
use crate::repositories::memberRepository::get_member;
use crate::repositories::projectRepository::get_project_by_id;
use crate::repositories::sprintRepository::get_sprint_by_id;
use crate::repositories::ticketRepository::{
    create_ticket, delete_ticket, get_backlog_tickets, get_ticket_by_id,
    get_tickets_by_project_id, get_tickets_by_sprint_id, set_ticket_assignee_to_null,
    set_ticket_sprint_to_null, update_ticket, update_ticket_assignee, update_ticket_sprint,
    update_ticket_status,
};
use actix_web::error::{Error, ErrorBadRequest, ErrorNotFound};
use sqlx::MySqlPool;
use uuid::Uuid;

async fn ensure_project_member(
    pool: &MySqlPool,
    project_id: Uuid,
    user_id: Uuid,
) -> Result<Project, Error> {
    let project = get_project_by_id(pool, project_id)
        .await
        .map_err(|_| ErrorNotFound("Project not found"))?;

    if project.owner_id == user_id {
        return Ok(project);
    }

    get_member(pool, user_id, project_id)
        .await
        .map_err(|_| {
            actix_web::error::ErrorUnauthorized("You are not a member of this project".to_string())
        })?;

    Ok(project)
}

async fn get_project_ticket(
    pool: &MySqlPool,
    project_id: Uuid,
    ticket_id: Uuid,
) -> Result<Ticket, Error> {
    let ticket = get_ticket_by_id(pool, ticket_id)
        .await
        .map_err(|_| ErrorNotFound("Ticket not found"))?;

    if ticket.project_id != project_id {
        return Err(ErrorBadRequest(
            "Ticket not found for this project".to_string(),
        ));
    }

    Ok(ticket)
}

pub async fn createTicketService(
    pool: &MySqlPool,
    project_id: Uuid,
    ticket: CreateTicketRequest,
    user_id: Uuid,
) -> Result<TicketResponse, Error> {
    ensure_project_member(pool, project_id, user_id).await?;

    let description = ticket.description.as_deref().unwrap_or("");
    let ticket = create_ticket(
        pool,
        Uuid::new_v4(),
        project_id,
        None,
        &ticket.title,
        description,
        user_id,
        None,
        TicketStatus::Backlog,
        ticket.priority,
    )
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(TicketResponse::from(ticket))
}

pub async fn updateTicketService(
    pool: &MySqlPool,
    project_id: Uuid,
    ticket_id: Uuid,
    ticket: UpdateTicketRequest,
    user_id: Uuid,
) -> Result<TicketResponse, Error> {
    ensure_project_member(pool, project_id, user_id).await?;
    let current = get_project_ticket(pool, project_id, ticket_id).await?;

    let title = ticket.title.as_deref().unwrap_or(&current.title);
    let current_description = current.description.unwrap_or_default();
    let description = ticket
        .description
        .as_deref()
        .unwrap_or(&current_description);
    let priority = ticket.priority.unwrap_or(current.priority);

    update_ticket(pool, ticket_id, title, description, priority)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let updated = get_ticket_by_id(pool, ticket_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(TicketResponse::from(updated))
}

pub async fn assignTicketService(
    pool: &MySqlPool,
    project_id: Uuid,
    ticket_id: Uuid,
    request: UpdateTicketAssigneeRequest,
    user_id: Uuid,
) -> Result<TicketResponse, Error> {
    ensure_project_member(pool, project_id, user_id).await?;
    get_project_ticket(pool, project_id, ticket_id).await?;

    match request.assignee_id {
        Some(assignee_id) => {
            get_member(pool, assignee_id, project_id)
                .await
                .map_err(|_| ErrorBadRequest("Assignee is not a member of this project".to_string()))?;

            update_ticket_assignee(pool, ticket_id, assignee_id)
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;
        }
        None => {
            set_ticket_assignee_to_null(pool, ticket_id)
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;
        }
    }

    let updated = get_ticket_by_id(pool, ticket_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(TicketResponse::from(updated))
}

pub async fn updateTicketStatusService(
    pool: &MySqlPool,
    project_id: Uuid,
    ticket_id: Uuid,
    request: UpdateTicketStatusRequest,
    user_id: Uuid,
) -> Result<TicketResponse, Error> {
    ensure_project_member(pool, project_id, user_id).await?;
    get_project_ticket(pool, project_id, ticket_id).await?;

    update_ticket_status(pool, ticket_id, request.status)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let updated = get_ticket_by_id(pool, ticket_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(TicketResponse::from(updated))
}

pub async fn assignTicketSprintService(
    pool: &MySqlPool,
    project_id: Uuid,
    ticket_id: Uuid,
    request: UpdateTicketSprintRequest,
    user_id: Uuid,
) -> Result<TicketResponse, Error> {
    ensure_project_member(pool, project_id, user_id).await?;
    get_project_ticket(pool, project_id, ticket_id).await?;

    match request.sprint_id {
        Some(sprint_id) => {
            let sprint = get_sprint_by_id(pool, sprint_id)
                .await
                .map_err(|_| ErrorNotFound("Sprint not found"))?;

            if sprint.project_id != project_id {
                return Err(ErrorBadRequest(
                    "Sprint not found for this project".to_string(),
                ));
            }

            update_ticket_sprint(pool, ticket_id, sprint_id)
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;
        }
        None => {
            set_ticket_sprint_to_null(pool, ticket_id)
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;
        }
    }

    let updated = get_ticket_by_id(pool, ticket_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(TicketResponse::from(updated))
}

pub async fn deleteTicketService(
    pool: &MySqlPool,
    project_id: Uuid,
    ticket_id: Uuid,
    user_id: Uuid,
) -> Result<(), Error> {
    ensure_project_member(pool, project_id, user_id).await?;
    get_project_ticket(pool, project_id, ticket_id).await?;

    delete_ticket(pool, ticket_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(())
}

pub async fn getProjectTicketsService(
    pool: &MySqlPool,
    project_id: Uuid,
    user_id: Uuid,
) -> Result<Vec<TicketResponse>, Error> {
    ensure_project_member(pool, project_id, user_id).await?;

    let tickets = get_tickets_by_project_id(pool, project_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(tickets.into_iter().map(TicketResponse::from).collect())
}

pub async fn getSprintTicketsService(
    pool: &MySqlPool,
    project_id: Uuid,
    sprint_id: Uuid,
    user_id: Uuid,
) -> Result<Vec<TicketResponse>, Error> {
    ensure_project_member(pool, project_id, user_id).await?;

    let sprint = get_sprint_by_id(pool, sprint_id)
        .await
        .map_err(|_| ErrorNotFound("Sprint not found"))?;

    if sprint.project_id != project_id {
        return Err(ErrorBadRequest(
            "Sprint not found for this project".to_string(),
        ));
    }

    let tickets = get_tickets_by_sprint_id(pool, sprint_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(tickets.into_iter().map(TicketResponse::from).collect())
}

pub async fn getBacklogTicketsService(
    pool: &MySqlPool,
    project_id: Uuid,
    user_id: Uuid,
) -> Result<Vec<TicketResponse>, Error> {
    ensure_project_member(pool, project_id, user_id).await?;

    let tickets = get_backlog_tickets(pool, project_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(tickets.into_iter().map(TicketResponse::from).collect())
}
