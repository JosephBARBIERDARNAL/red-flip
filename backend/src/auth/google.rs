use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::auth::jwt::create_token;
use crate::config::AppConfig;
use crate::errors::AppError;
use crate::models::user::User;

#[derive(Deserialize)]
pub struct GoogleCallbackQuery {
    pub code: String,
}

#[derive(Deserialize)]
struct GoogleTokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct GoogleUserInfo {
    id: String,
    email: String,
    name: String,
    picture: Option<String>,
}

pub async fn google_login(config: web::Data<AppConfig>) -> Result<HttpResponse, AppError> {
    let client_id = config
        .google_client_id
        .as_ref()
        .ok_or_else(|| AppError::BadRequest("Google OAuth not configured".into()))?;
    let redirect_uri = config
        .google_redirect_uri
        .as_ref()
        .ok_or_else(|| AppError::BadRequest("Google OAuth not configured".into()))?;

    let url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope=openid%20email%20profile",
        client_id, redirect_uri
    );

    Ok(HttpResponse::Found()
        .append_header(("Location", url))
        .finish())
}

pub async fn google_callback(
    pool: web::Data<SqlitePool>,
    config: web::Data<AppConfig>,
    query: web::Query<GoogleCallbackQuery>,
) -> Result<HttpResponse, AppError> {
    let client_id = config
        .google_client_id
        .as_ref()
        .ok_or_else(|| AppError::Internal("Google OAuth not configured".into()))?;
    let client_secret = config
        .google_client_secret
        .as_ref()
        .ok_or_else(|| AppError::Internal("Google OAuth not configured".into()))?;
    let redirect_uri = config
        .google_redirect_uri
        .as_ref()
        .ok_or_else(|| AppError::Internal("Google OAuth not configured".into()))?;

    let client = reqwest::Client::new();

    // Exchange code for token
    let token_res = client
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("code", query.code.as_str()),
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("redirect_uri", redirect_uri.as_str()),
            ("grant_type", "authorization_code"),
        ])
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Google token exchange failed: {e}")))?
        .json::<GoogleTokenResponse>()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to parse Google token response: {e}")))?;

    // Fetch user info
    let user_info = client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(&token_res.access_token)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to fetch Google user info: {e}")))?
        .json::<GoogleUserInfo>()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to parse Google user info: {e}")))?;

    // Find or create user
    let user = if let Some(user) = User::find_by_google_id(&pool, &user_info.id).await? {
        user
    } else if let Some(user) = User::find_by_email(&pool, &user_info.email).await? {
        // Link Google account to existing user (could extend this)
        user
    } else {
        // Generate unique username from Google name
        let base_username = user_info
            .name
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_lowercase();
        let username = if base_username.len() >= 3 {
            base_username[..base_username.len().min(16)].to_string()
        } else {
            format!("user_{}", &user_info.id[..8.min(user_info.id.len())])
        };

        User::create_from_google(
            &pool,
            &username,
            &user_info.email,
            &user_info.id,
            user_info.picture.as_deref(),
        )
        .await?
    };

    let token = create_token(&user.id, &config.jwt_secret)?;

    // Redirect to frontend with token
    let redirect_url = format!("{}/?token={}", config.frontend_url, token);
    Ok(HttpResponse::Found()
        .append_header(("Location", redirect_url))
        .finish())
}
