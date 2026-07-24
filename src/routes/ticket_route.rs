use actix_web::web::{scope, ServiceConfig};

use crate::handlers::ticket_handlers::{
    assignTicket, assignTicketSprint, createTicket, deleteTicket, getBacklogTickets,
    getProjectTickets, getSprintTickets, updateTicket, updateTicketStatus,
};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/ticket")
            .service(createTicket)
            .service(getBacklogTickets)
            .service(getSprintTickets)
            .service(getProjectTickets)
            .service(updateTicket)
            .service(assignTicket)
            .service(updateTicketStatus)
            .service(assignTicketSprint)
            .service(deleteTicket),
    );
}
