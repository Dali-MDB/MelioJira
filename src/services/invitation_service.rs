use crate::dtos::invitation::{CreateInvitationRequest, InvitationResponse};
use crate::enums::invitation_status::InvitationStatus;
use crate::enums::user_role::Role;
use crate::models::invitation::Invitation;
use crate::repositories::inviteRepository::{
    create_invite, get_invite_by_id, get_invite_by_user_id, get_invites_by_project_id,
    update_status,
};
use crate::repositories::memberRepository::{create_member, get_member};
use crate::repositories::projectRepository::get_project_by_id;
use crate::repositories::userRepository::exists_by_id;
use actix_web::error::{Error, ErrorBadRequest, ErrorNotFound};
use sqlx::MySqlPool;
use uuid::Uuid;

pub async fn inviteUserService(
    pool: &MySqlPool,
    project_id: Uuid,
    request: CreateInvitationRequest,
    user_id: Uuid,
) -> Result<InvitationResponse, Error> {
    if request.invited_user_id == user_id {
        return Err(ErrorBadRequest("You cannot invite yourself".to_string()));
    }

    let project = get_project_by_id(pool, project_id)
        .await
        .map_err(|_| ErrorNotFound("Project not found"))?;
    if project.owner_id != user_id {
        return Err(actix_web::error::ErrorUnauthorized(
            "You are not the owner of this project".to_string(),
        ));
    }

    if !exists_by_id(pool, request.invited_user_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?
    {
        return Err(ErrorNotFound("User not found".to_string()));
    }

    if get_member(pool, request.invited_user_id, project_id)
        .await
        .is_ok()
    {
        return Err(ErrorBadRequest("User is already a member".to_string()));
    }

    let invites = get_invites_by_project_id(pool, project_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    if invites.iter().any(|invite| {
        invite.invited_user_id == request.invited_user_id
            && invite.status == InvitationStatus::Pending
    }) {
        return Err(ErrorBadRequest("Invitation already pending".to_string()));
    }

    let invitation = create_invite(pool, project_id, request.invited_user_id, user_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(InvitationResponse::from(invitation))
}

pub async fn acceptInvitationService(
    pool: &MySqlPool,
    invitation_id: Uuid,
    user_id: Uuid,
) -> Result<InvitationResponse, Error> {
    let invitation = get_invite_by_id(pool, invitation_id)
        .await
        .map_err(|_| ErrorNotFound("Invitation not found"))?;

    if invitation.invited_user_id != user_id {
        return Err(actix_web::error::ErrorUnauthorized(
            "You are not the invited user".to_string(),
        ));
    }
    if invitation.status != InvitationStatus::Pending {
        return Err(ErrorBadRequest("Invitation is not pending".to_string()));
    }

    update_status(pool, invitation_id, InvitationStatus::Accepted)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    create_member(pool, user_id, invitation.project_id, Role::Member)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let updated = Invitation {
        status: InvitationStatus::Accepted,
        ..invitation
    };
    Ok(InvitationResponse::from(updated))
}

pub async fn cancelInvitationService(
    pool: &MySqlPool,
    invitation_id: Uuid,
    user_id: Uuid,
) -> Result<InvitationResponse, Error> {
    let invitation = get_invite_by_id(pool, invitation_id)
        .await
        .map_err(|_| ErrorNotFound("Invitation not found"))?;
    let project = get_project_by_id(pool, invitation.project_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if project.owner_id != user_id {
        return Err(actix_web::error::ErrorUnauthorized(
            "You are not the owner of this project".to_string(),
        ));
    }
    if invitation.status != InvitationStatus::Pending {
        return Err(ErrorBadRequest("Invitation is not pending".to_string()));
    }

    update_status(pool, invitation_id, InvitationStatus::Cancelled)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let updated = Invitation {
        status: InvitationStatus::Cancelled,
        ..invitation
    };
    Ok(InvitationResponse::from(updated))
}

pub async fn rejectInvitationService(
    pool: &MySqlPool,
    invitation_id: Uuid,
    user_id: Uuid,
) -> Result<InvitationResponse, Error> {
    let invitation = get_invite_by_id(pool, invitation_id)
        .await
        .map_err(|_| ErrorNotFound("Invitation not found"))?;

    if invitation.invited_user_id != user_id {
        return Err(actix_web::error::ErrorUnauthorized(
            "You are not the invited user".to_string(),
        ));
    }
    if invitation.status != InvitationStatus::Pending {
        return Err(ErrorBadRequest("Invitation is not pending".to_string()));
    }

    update_status(pool, invitation_id, InvitationStatus::Rejected)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let updated = Invitation {
        status: InvitationStatus::Rejected,
        ..invitation
    };
    Ok(InvitationResponse::from(updated))
}

pub async fn viewMyInvitationsService(
    pool: &MySqlPool,
    user_id: Uuid,
) -> Result<Vec<InvitationResponse>, Error> {
    let invitations = get_invite_by_user_id(pool, user_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let responses = invitations
        .into_iter()
        .map(InvitationResponse::from)
        .collect();
    Ok(responses)
}

pub async fn viewProjectInvitationsService(
    pool: &MySqlPool,
    project_id: Uuid,
    user_id: Uuid,
) -> Result<Vec<InvitationResponse>, Error> {
    let project = get_project_by_id(pool, project_id)
        .await
        .map_err(|_| ErrorNotFound("Project not found"))?;
    if project.owner_id != user_id {
        return Err(actix_web::error::ErrorUnauthorized(
            "You are not the owner of this project".to_string(),
        ));
    }

    let invitations = get_invites_by_project_id(pool, project_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let responses = invitations
        .into_iter()
        .map(InvitationResponse::from)
        .collect();
    Ok(responses)
}
