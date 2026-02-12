use actix_web::{web, HttpResponse};

use crate::db::Database;
use crate::errors::AppError;
use crate::models::user::{PublicUser, User};

pub async fn get_leaderboard(db: web::Data<Database>) -> Result<HttpResponse, AppError> {
    let users = User::top_by_elo(&db, 10).await?;
    let public_users: Vec<PublicUser> = users.into_iter().map(PublicUser::from).collect();

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "leaderboard": public_users,
    })))
}
