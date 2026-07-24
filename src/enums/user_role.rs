use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Copy,  Type)]
#[sqlx(rename_all = "PascalCase")]
pub enum Role {
    Owner,
    Admin,
    Member,
}
