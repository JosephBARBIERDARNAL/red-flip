use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use crate::api::{admin, dashboard, leaderboard, user};
use crate::auth::middleware::extract_user_from_query;
use crate::auth::{google, handlers};
use crate::config::AppConfig;
use crate::db::Database;
use crate::game::matchmaking::MatchmakingActor;
use crate::game::ws::PlayerWsActor;
use crate::models::user::User;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(health))
            .route("/leaderboard", web::get().to(leaderboard::get_leaderboard))
            .route("/dashboard", web::get().to(dashboard::get_dashboard))
            .route("/users/{id}", web::get().to(user::get_user))
            .route("/account/delete", web::delete().to(user::delete_account))
            .service(
                web::scope("/admin")
                    .route("/stats", web::get().to(admin::get_stats))
                    .route("/users", web::get().to(admin::list_users))
                    .route("/users/{id}", web::put().to(admin::update_user))
                    .route("/users/{id}/ban", web::post().to(admin::ban_user))
                    .route("/users/{id}/unban", web::post().to(admin::unban_user))
                    .route("/users/{id}", web::delete().to(admin::delete_user)),
            ),
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
    db: web::Data<Database>,
    config: web::Data<AppConfig>,
    matchmaking: web::Data<actix::Addr<MatchmakingActor>>,
) -> Result<HttpResponse, actix_web::Error> {
    let query = req.query_string();
    let user_id = extract_user_from_query(query, &config.jwt_secret)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    let user = User::find_by_id(&db, &user_id)
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
