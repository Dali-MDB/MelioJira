use crate::dtos::ticket::{
    CreateTicketRequest, UpdateTicketAssigneeRequest, UpdateTicketRequest,
    UpdateTicketSprintRequest, UpdateTicketStatusRequest,
};
use crate::models::user::User;
use crate::services::ticket_services::{
    assignTicketService, assignTicketSprintService, createTicketService, deleteTicketService,
    getBacklogTicketsService, getProjectTicketsService, getSprintTicketsService,
    updateTicketService, updateTicketStatusService,
};
use crate::state::AppState;
use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse, delete, get, post, put, web};
use uuid::Uuid;

#[post("/tickets/{project_id}")]
pub async fn createTicket(
    req: HttpRequest,
    state: web::Data<AppState>,
    project_id: web::Path<Uuid>,
    ticket: web::Json<CreateTicketRequest>,
) -> Result<HttpResponse, Error> {
    let extensions = req.extensions_mut();
    let current_user = extensions
        .get::<User>()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Unauthorized"))?;

    let response = createTicketService(
        &state.pool,
        project_id.into_inner(),
        ticket.into_inner(),
        current_user.id,
    )
    .await?;

    Ok(HttpResponse::Created().json(response))
}

#[put("/tickets/{project_id}/{ticket_id}")]
pub async fn updateTicket(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
    ticket: web::Json<UpdateTicketRequest>,
) -> Result<HttpResponse, Error> {
    let extensions = req.extensions_mut();
    let current_user = extensions
        .get::<User>()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Unauthorized"))?;

    let (project_id, ticket_id) = path.into_inner();
    let response = updateTicketService(
        &state.pool,
        project_id,
        ticket_id,
        ticket.into_inner(),
        current_user.id,
    )
    .await?;

    Ok(HttpResponse::Ok().json(response))
}

#[put("/tickets/{project_id}/{ticket_id}/assignee")]
pub async fn assignTicket(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<UpdateTicketAssigneeRequest>,
) -> Result<HttpResponse, Error> {
    let extensions = req.extensions_mut();
    let current_user = extensions
        .get::<User>()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Unauthorized"))?;

    let (project_id, ticket_id) = path.into_inner();
    let response = assignTicketService(
        &state.pool,
        project_id,
        ticket_id,
        body.into_inner(),
        current_user.id,
    )
    .await?;

    Ok(HttpResponse::Ok().json(response))
}

#[put("/tickets/{project_id}/{ticket_id}/status")]
pub async fn updateTicketStatus(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<UpdateTicketStatusRequest>,
) -> Result<HttpResponse, Error> {
    let extensions = req.extensions_mut();
    let current_user = extensions
        .get::<User>()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Unauthorized"))?;

    let (project_id, ticket_id) = path.into_inner();
    let response = updateTicketStatusService(
        &state.pool,
        project_id,
        ticket_id,
        body.into_inner(),
        current_user.id,
    )
    .await?;

    Ok(HttpResponse::Ok().json(response))
}

#[put("/tickets/{project_id}/{ticket_id}/sprint")]
pub async fn assignTicketSprint(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<UpdateTicketSprintRequest>,
) -> Result<HttpResponse, Error> {
    let extensions = req.extensions_mut();
    let current_user = extensions
        .get::<User>()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Unauthorized"))?;

    let (project_id, ticket_id) = path.into_inner();
    let response = assignTicketSprintService(
        &state.pool,
        project_id,
        ticket_id,
        body.into_inner(),
        current_user.id,
    )
    .await?;

    Ok(HttpResponse::Ok().json(response))
}

#[delete("/tickets/{project_id}/{ticket_id}")]
pub async fn deleteTicket(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, Error> {
    let extensions = req.extensions_mut();
    let current_user = extensions
        .get::<User>()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Unauthorized"))?;

    let (project_id, ticket_id) = path.into_inner();
    deleteTicketService(&state.pool, project_id, ticket_id, current_user.id).await?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/tickets/{project_id}")]
pub async fn getProjectTickets(
    req: HttpRequest,
    state: web::Data<AppState>,
    project_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let extensions = req.extensions_mut();
    let current_user = extensions
        .get::<User>()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Unauthorized"))?;

    let response = getProjectTicketsService(
        &state.pool,
        project_id.into_inner(),
        current_user.id,
    )
    .await?;

    Ok(HttpResponse::Ok().json(response))
}

#[get("/tickets/{project_id}/sprint/{sprint_id}")]
pub async fn getSprintTickets(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, Error> {
    let extensions = req.extensions_mut();
    let current_user = extensions
        .get::<User>()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Unauthorized"))?;

    let (project_id, sprint_id) = path.into_inner();
    let response = getSprintTicketsService(
        &state.pool,
        project_id,
        sprint_id,
        current_user.id,
    )
    .await?;

    Ok(HttpResponse::Ok().json(response))
}

#[get("/tickets/{project_id}/backlog")]
pub async fn getBacklogTickets(
    req: HttpRequest,
    state: web::Data<AppState>,
    project_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let extensions = req.extensions_mut();
    let current_user = extensions
        .get::<User>()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Unauthorized"))?;

    let response = getBacklogTicketsService(
        &state.pool,
        project_id.into_inner(),
        current_user.id,
    )
    .await?;

    Ok(HttpResponse::Ok().json(response))
}
