use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::auth::middleware::AuthenticatedUser;
use crate::db::Database;
use crate::errors::AppError;
use crate::models::user::{PlatformStats, User};

// Helper to check admin access
async fn require_admin(db: &Database, user_id: &str) -> Result<(), AppError> {
    if !User::is_admin(db, user_id).await? {
        return Err(AppError::Unauthorized("Admin access required".into()));
    }
    Ok(())
}

#[derive(Serialize)]
pub struct AdminStatsResponse {
    pub stats: PlatformStats,
}

#[derive(Serialize)]
pub struct AdminUsersResponse {
    pub users: Vec<User>,
    pub total: i64,
    pub page: i32,
    pub limit: i32,
}

#[derive(Deserialize)]
pub struct ListUsersQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub elo: Option<i32>,
    pub wins: Option<i32>,
    pub losses: Option<i32>,
    pub draws: Option<i32>,
}

#[derive(Deserialize)]
pub struct BanUserRequest {
    pub reason: String,
}

pub async fn get_stats(
    db: web::Data<Database>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    require_admin(&db, &auth.user_id).await?;

    let stats = User::get_platform_stats(&db).await?;

    Ok(HttpResponse::Ok().json(AdminStatsResponse { stats }))
}

pub async fn list_users(
    db: web::Data<Database>,
    auth: AuthenticatedUser,
    query: web::Query<ListUsersQuery>,
) -> Result<HttpResponse, AppError> {
    require_admin(&db, &auth.user_id).await?;

    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(20).min(100).max(1);
    let offset = (page - 1) * limit;

    let search = query.search.as_deref();
    let sort_by = query.sort_by.as_deref();

    let users = User::list_with_filters(&db, search, sort_by, offset, limit).await?;
    let total = User::count_all(&db, search).await?;

    Ok(HttpResponse::Ok().json(AdminUsersResponse {
        users,
        total,
        page,
        limit,
    }))
}

pub async fn update_user(
    db: web::Data<Database>,
    auth: AuthenticatedUser,
    user_id: web::Path<String>,
    body: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse, AppError> {
    require_admin(&db, &auth.user_id).await?;

    // Prevent editing self
    if &auth.user_id == user_id.as_str() {
        return Err(AppError::BadRequest("Cannot edit your own account".into()));
    }

    // Prevent editing other admins
    let target_user = User::find_by_id(&db, &user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    if target_user.is_admin {
        return Err(AppError::BadRequest("Cannot edit admin accounts".into()));
    }

    // Validate inputs
    if let Some(ref username) = body.username {
        if username.len() < 3 || username.len() > 20 {
            return Err(AppError::BadRequest(
                "Username must be between 3 and 20 characters".into(),
            ));
        }
        if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(AppError::BadRequest(
                "Username can only contain alphanumeric characters and underscores".into(),
            ));
        }
    }

    if let Some(elo) = body.elo {
        if elo < 0 || elo > 5000 {
            return Err(AppError::BadRequest(
                "Elo must be between 0 and 5000".into(),
            ));
        }
    }

    if let Some(wins) = body.wins {
        if wins < 0 {
            return Err(AppError::BadRequest("Wins cannot be negative".into()));
        }
    }

    if let Some(losses) = body.losses {
        if losses < 0 {
            return Err(AppError::BadRequest("Losses cannot be negative".into()));
        }
    }

    if let Some(draws) = body.draws {
        if draws < 0 {
            return Err(AppError::BadRequest("Draws cannot be negative".into()));
        }
    }

    User::update_stats(
        &db,
        &user_id,
        body.username.as_deref(),
        body.elo,
        body.wins,
        body.losses,
        body.draws,
    )
    .await?;

    let updated_user = User::find_by_id(&db, &user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    Ok(HttpResponse::Ok().json(updated_user))
}

pub async fn ban_user(
    db: web::Data<Database>,
    auth: AuthenticatedUser,
    user_id: web::Path<String>,
    body: web::Json<BanUserRequest>,
) -> Result<HttpResponse, AppError> {
    require_admin(&db, &auth.user_id).await?;

    // Prevent banning self
    if &auth.user_id == user_id.as_str() {
        return Err(AppError::BadRequest("Cannot ban yourself".into()));
    }

    // Prevent banning other admins
    let target_user = User::find_by_id(&db, &user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    if target_user.is_admin {
        return Err(AppError::BadRequest("Cannot ban admin accounts".into()));
    }

    if body.reason.trim().is_empty() {
        return Err(AppError::BadRequest("Ban reason is required".into()));
    }

    if body.reason.len() > 500 {
        return Err(AppError::BadRequest(
            "Ban reason must be less than 500 characters".into(),
        ));
    }

    User::ban(&db, &user_id, &body.reason).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "User banned successfully"
    })))
}

pub async fn unban_user(
    db: web::Data<Database>,
    auth: AuthenticatedUser,
    user_id: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    require_admin(&db, &auth.user_id).await?;

    let target_user = User::find_by_id(&db, &user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    if !target_user.is_banned {
        return Err(AppError::BadRequest("User is not banned".into()));
    }

    User::unban(&db, &user_id).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "User unbanned successfully"
    })))
}

pub async fn delete_user(
    db: web::Data<Database>,
    auth: AuthenticatedUser,
    user_id: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    require_admin(&db, &auth.user_id).await?;

    // Prevent deleting self
    if &auth.user_id == user_id.as_str() {
        return Err(AppError::BadRequest("Cannot delete yourself".into()));
    }

    // Prevent deleting other admins
    let target_user = User::find_by_id(&db, &user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    if target_user.is_admin {
        return Err(AppError::BadRequest("Cannot delete admin accounts".into()));
    }

    User::delete(&db, &user_id).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "User deleted successfully"
    })))
}
