use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;

use crate::errors::AppError;
use crate::models::user::{PublicUser, User};

pub async fn get_leaderboard(pool: web::Data<SqlitePool>) -> Result<HttpResponse, AppError> {
    let users = User::top_by_elo(&pool, 10).await?;
    let public_users: Vec<PublicUser> = users.into_iter().map(PublicUser::from).collect();

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "leaderboard": public_users,
    })))
}
