use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;

use crate::auth::middleware::AuthenticatedUser;
use crate::errors::AppError;
use crate::models::match_record::MatchRecord;
use crate::models::user::{PublicUser, User};

pub async fn get_dashboard(
    pool: web::Data<SqlitePool>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let user = User::find_by_id(&pool, &auth.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    let recent_matches = MatchRecord::recent_for_user(&pool, &auth.user_id, 10).await?;

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
