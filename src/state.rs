use crate::config::Config;
use sqlx::MySqlPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
    pub config: Config,
}
