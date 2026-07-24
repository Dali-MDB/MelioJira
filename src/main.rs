use actix_web::{App, HttpServer, web};
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
mod middlewares;
mod repositories;

mod handlers;
mod routes;
mod services;

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
        config: my_config,
    };
    let port = my_app_state.config.port.parse::<u16>().unwrap();
    let app_data = web::Data::new(my_app_state);
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(routes::auth_route::config)
            .configure(routes::project_route::config)
            .configure(routes::invitation_route::config)
            .configure(routes::ticket_route::config)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
