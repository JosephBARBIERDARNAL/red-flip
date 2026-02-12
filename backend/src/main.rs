mod api;
mod auth;
mod config;
mod db;
mod errors;
mod game;
mod models;
mod routes;

use actix::Actor;
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};

use config::AppConfig;
use game::matchmaking::MatchmakingActor;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let config = AppConfig::from_env();
    let port = config.backend_port;
    let frontend_url = config.frontend_url.clone();

    let pool = db::init_pool(&config.database_url).await;
    db::run_migrations(&pool).await;

    let matchmaking = MatchmakingActor::new(pool.clone()).start();

    log::info!("Starting server on port {port}");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&frontend_url)
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec!["Content-Type", "Authorization"])
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(matchmaking.clone()))
            .configure(routes::configure)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
