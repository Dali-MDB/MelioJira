use crate::dtos::invitation::CreateInvitationRequest;
use crate::models::user::User;
use crate::services::invitation_service::{
    acceptInvitationService, cancelInvitationService, inviteUserService,
    rejectInvitationService, viewMyInvitationsService, viewProjectInvitationsService,
};
use crate::state::AppState;
use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse, get, post, put, web};
use uuid::Uuid;

#[post("/{project_id}")]
pub async fn inviteUser(
    req: HttpRequest,
    state: web::Data<AppState>,
    project_id: web::Path<Uuid>,
    body: web::Json<CreateInvitationRequest>,
) -> Result<HttpResponse, Error> {
    let extension = req.extensions_mut();
    let current_user = extension
        .get::<User>()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Unauthorized"))?;
    let response = inviteUserService(
        &state.pool,
        project_id.into_inner(),
        body.into_inner(),
        current_user.id,
    )
    .await?;
    Ok(HttpResponse::Created().json(response))
}

#[get("/me")]
pub async fn viewMyInvitations(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let extension = req.extensions_mut();
    let current_user = extension
        .get::<User>()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Unauthorized"))?;
    let response = viewMyInvitationsService(&state.pool, current_user.id).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[get("/project/{project_id}")]
pub async fn viewProjectInvitations(
    req: HttpRequest,
    state: web::Data<AppState>,
    project_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let extension = req.extensions_mut();
    let current_user = extension
        .get::<User>()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Unauthorized"))?;
    let response = viewProjectInvitationsService(
        &state.pool,
        project_id.into_inner(),
        current_user.id,
    )
    .await?;
    Ok(HttpResponse::Ok().json(response))
}

#[put("/{invitation_id}/accept")]
pub async fn acceptInvitation(
    req: HttpRequest,
    state: web::Data<AppState>,
    invitation_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let extension = req.extensions_mut();
    let current_user = extension
        .get::<User>()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Unauthorized"))?;
    let response =
        acceptInvitationService(&state.pool, invitation_id.into_inner(), current_user.id).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[put("/{invitation_id}/cancel")]
pub async fn cancelInvitation(
    req: HttpRequest,
    state: web::Data<AppState>,
    invitation_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let extension = req.extensions_mut();
    let current_user = extension
        .get::<User>()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Unauthorized"))?;
    let response =
        cancelInvitationService(&state.pool, invitation_id.into_inner(), current_user.id).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[put("/{invitation_id}/reject")]
pub async fn rejectInvitation(
    req: HttpRequest,
    state: web::Data<AppState>,
    invitation_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let extension = req.extensions_mut();
    let current_user = extension
        .get::<User>()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Unauthorized"))?;
    let response =
        rejectInvitationService(&state.pool, invitation_id.into_inner(), current_user.id).await?;
    Ok(HttpResponse::Ok().json(response))
}
