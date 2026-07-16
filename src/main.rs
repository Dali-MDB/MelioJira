use actix_web::{App, HttpServer};
use dotenvy;

mod state;
use sqlx::MySqlPool;
use state::AppState;

mod config;
use config::Config;

mod auth;
mod enums;
mod models;

mod dtos;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok(); //load env variables
    let my_config = Config {
        database_url: std::env::var("database_url").expect("database_url should be set"),
        jwt_secret: std::env::var("jwt_secret").expect("jwt_secret should be set"),
        port: std::env::var("port").expect("port should be set"),
    };

    let my_app_state = AppState {
        pool: MySqlPool::connect(&my_config.database_url)
            .await
            .expect("couldn't connect to MySql"),
    };
    HttpServer::new(move || App::new().app_data(my_app_state.clone()))
        .bind(("127.0.0.1", my_config.port.parse::<u16>().unwrap()))?
        .run()
        .await
}
