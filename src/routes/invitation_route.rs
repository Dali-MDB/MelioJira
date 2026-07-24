use actix_web::web::{scope, ServiceConfig};

use crate::handlers::invitation_handlers::{
    acceptInvitation, cancelInvitation, inviteUser, rejectInvitation, viewMyInvitations,
    viewProjectInvitations,
};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/Invitations")
            .service(inviteUser)
            .service(viewMyInvitations)
            .service(viewProjectInvitations)
            .service(acceptInvitation)
            .service(cancelInvitation)
            .service(rejectInvitation),
    );
}
