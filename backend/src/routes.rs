use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use uuid::Uuid;

use crate::api::{admin, dashboard, leaderboard, user};
use crate::auth::handlers;
use crate::auth::middleware::extract_optional_user_from_query;
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
            .route("/me", web::get().to(handlers::me)),
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
    let user_id_opt = extract_optional_user_from_query(query, &config.jwt_secret);

    let (user_id, username, elo, is_guest) = if let Some(uid) = user_id_opt {
        // Authenticated user
        let user = User::find_by_id(&db, &uid)
            .await
            .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
            .ok_or_else(|| actix_web::error::ErrorNotFound("User not found"))?;

        (user.id, user.username, user.elo, false)
    } else {
        // Guest user
        let guest_id = format!("guest_{}", Uuid::new_v4());
        let guest_name = format!("Guest{}", &guest_id[6..10]);
        (guest_id, guest_name, 1000, true)
    };

    let actor = PlayerWsActor::new(
        user_id,
        username,
        elo,
        is_guest,
        matchmaking.get_ref().clone(),
    );

    ws::start(actor, &req, stream)
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test, App};

    use super::configure;

    #[actix_rt::test]
    async fn health_endpoint_returns_ok_status() {
        let app = test::init_service(App::new().configure(configure)).await;
        let req = test::TestRequest::get().uri("/api/health").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body["status"], "ok");
    }
}
