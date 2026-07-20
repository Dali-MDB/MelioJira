use futures_util::future::ok;
use sqlx::MySqlPool;
use uuid::Uuid;
use crate::models::user::User;

pub async fn fetch_user_by_id(pool: &MySqlPool, id:Uuid)->Result<User, sqlx::Error>{
    //
}