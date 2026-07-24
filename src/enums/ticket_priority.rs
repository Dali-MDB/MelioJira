use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Copy, sqlx::Type)]
#[sqlx(rename_all = "PascalCase")]
pub enum TicketPriority {
    Low,
    Medium,
    High,
}
