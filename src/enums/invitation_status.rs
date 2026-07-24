use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Type)]
#[sqlx(rename_all = "PascalCase")]
pub enum InvitationStatus {
    Pending,
    Cancelled,
    Accepted,
    Rejected,
}
