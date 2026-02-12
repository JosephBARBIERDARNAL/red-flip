use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use sqlx::SqlitePool;

use crate::api::{dashboard, leaderboard, user};
use crate::auth::{google, handlers};
use crate::auth::middleware::extract_user_from_query;
use crate::config::AppConfig;
use crate::game::matchmaking::MatchmakingActor;
use crate::game::ws::PlayerWsActor;
use crate::models::user::User;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(health))
            .route("/leaderboard", web::get().to(leaderboard::get_leaderboard))
            .route("/dashboard", web::get().to(dashboard::get_dashboard))
            .route("/users/{id}", web::get().to(user::get_user)),
    )
    .service(
        web::scope("/auth")
            .route("/register", web::post().to(handlers::register))
            .route("/login", web::post().to(handlers::login))
            .route("/me", web::get().to(handlers::me))
            .route("/google", web::get().to(google::google_login))
            .route("/google/callback", web::get().to(google::google_callback)),
    )
    .route("/ws", web::get().to(ws_handler));
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

async fn ws_handler(
    req: HttpRequest,
    stream: web::Payload,
    pool: web::Data<SqlitePool>,
    config: web::Data<AppConfig>,
    matchmaking: web::Data<actix::Addr<MatchmakingActor>>,
) -> Result<HttpResponse, actix_web::Error> {
    let query = req.query_string();
    let user_id = extract_user_from_query(query, &config.jwt_secret)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    let user = User::find_by_id(&pool, &user_id)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("User not found"))?;

    let actor = PlayerWsActor::new(
        user.id,
        user.username,
        user.elo,
        matchmaking.get_ref().clone(),
    );

    ws::start(actor, &req, stream)
}
