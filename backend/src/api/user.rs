use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;

use crate::auth::middleware::AuthenticatedUser;
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

pub async fn delete_account(
    pool: web::Data<SqlitePool>,
    auth_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    // Verify the user exists before deleting
    User::find_by_id(&pool, &auth_user.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    // Delete the user and all related data
    User::delete(&pool, &auth_user.user_id).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Account deleted successfully",
    })))
}
