use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;

use crate::errors::AppError;
use crate::models::user::{PublicUser, User};

pub async fn get_user(
    pool: web::Data<SqlitePool>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
    let user = User::find_by_id(&pool, &user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "user": PublicUser::from(user),
    })))
}
