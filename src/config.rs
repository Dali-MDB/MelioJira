#[derive(Clone)]
#[derive(Debug, FromRow)]
pub struct Config{
    pub database_url : String,
    pub jwt_secret : String,
    pub port : String
}