use actix_web::{web, HttpResponse};

use crate::auth::middleware::AuthenticatedUser;
use crate::db::Database;
use crate::errors::AppError;
use crate::models::match_record::MatchRecord;
use crate::models::user::{PublicUser, User};

pub async fn get_dashboard(
    db: web::Data<Database>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let user = User::find_by_id(&db, &auth.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    let recent_matches = MatchRecord::recent_for_user(&db, &auth.user_id, 10).await?;

    let win_rate = if user.total_games > 0 {
        (user.wins as f64 / user.total_games as f64 * 100.0).round()
    } else {
        0.0
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "user": PublicUser::from(user),
        "recent_matches": recent_matches,
        "win_rate": win_rate,
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::body::to_bytes;

    use crate::db::init_test_db;

    #[actix_rt::test]
    async fn dashboard_returns_not_found_when_user_missing() {
        let db = web::Data::new(init_test_db().await);
        let result = get_dashboard(
            db,
            AuthenticatedUser {
                user_id: "missing".into(),
            },
        )
        .await;
        assert!(matches!(result, Err(AppError::NotFound(_))));
    }

    #[actix_rt::test]
    async fn dashboard_computes_win_rate() {
        let db = web::Data::new(init_test_db().await);
        let user = User::create(&db, "dash_user", "dash@example.com", "hash")
            .await
            .expect("user should be created");
        User::update_stats(&db, &user.id, None, None, Some(3), Some(1), Some(0))
            .await
            .expect("stats update should succeed");

        let resp = get_dashboard(
            db,
            AuthenticatedUser {
                user_id: user.id.clone(),
            },
        )
        .await
        .expect("dashboard should succeed");

        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
        let body = to_bytes(resp.into_body())
            .await
            .expect("response body should be readable");
        let json: serde_json::Value =
            serde_json::from_slice(&body).expect("response should be valid json");
        assert_eq!(json["win_rate"], 75.0);
    }
}
