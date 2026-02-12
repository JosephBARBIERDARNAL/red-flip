use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::auth::jwt::create_token;
use crate::auth::middleware::AuthenticatedUser;
use crate::config::AppConfig;
use crate::errors::AppError;
use crate::models::user::{PublicUser, User};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn register(
    pool: web::Data<SqlitePool>,
    config: web::Data<AppConfig>,
    body: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    if body.username.len() < 3 || body.username.len() > 20 {
        return Err(AppError::BadRequest(
            "Username must be between 3 and 20 characters".into(),
        ));
    }
    if body.password.len() < 6 {
        return Err(AppError::BadRequest(
            "Password must be at least 6 characters".into(),
        ));
    }

    let password_hash = bcrypt::hash(&body.password, 10)?;
    let user = User::create(&pool, &body.username, &body.email, &password_hash).await?;
    let token = create_token(&user.id, &config.jwt_secret)?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "token": token,
        "user": PublicUser::from(user),
    })))
}

pub async fn login(
    pool: web::Data<SqlitePool>,
    config: web::Data<AppConfig>,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let user = User::find_by_email(&pool, &body.email)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid email or password".into()))?;

    let password_hash = user
        .password_hash
        .as_ref()
        .ok_or_else(|| AppError::Unauthorized("This account uses Google sign-in".into()))?;

    if !bcrypt::verify(&body.password, password_hash)? {
        return Err(AppError::Unauthorized("Invalid email or password".into()));
    }

    let token = create_token(&user.id, &config.jwt_secret)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "token": token,
        "user": PublicUser::from(user),
    })))
}

pub async fn me(
    pool: web::Data<SqlitePool>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let user = User::find_by_id(&pool, &auth.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "user": PublicUser::from(user),
    })))
}
