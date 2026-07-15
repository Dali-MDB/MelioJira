use sqlx::MySqlPool;

#[derive(Clone)]
#[derive(Debug, FromRow)]
pub struct AppState{
    pub pool: MySqlPool,
}