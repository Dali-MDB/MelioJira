use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TicketStatus {
    Backlog,
    Todo,
    InProgress,
    Done,
}
