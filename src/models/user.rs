use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: Uuid,
    pub user_name: String,
    pub email: String,
    pub password: String,
}
