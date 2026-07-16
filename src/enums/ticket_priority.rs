use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TicketPriority {
    Low,
    Medium,
    High,
}
