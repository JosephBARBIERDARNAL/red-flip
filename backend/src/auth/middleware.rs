use actix_web::{dev::Payload, web, FromRequest, HttpRequest};
use std::future::{ready, Ready};

use crate::auth::jwt::validate_token;
use crate::config::AppConfig;
use crate::errors::AppError;

pub struct AuthenticatedUser {
    pub user_id: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let result = extract_user(req);
        ready(result)
    }
}

fn extract_user(req: &HttpRequest) -> Result<AuthenticatedUser, AppError> {
    let config = req
        .app_data::<web::Data<AppConfig>>()
        .ok_or_else(|| AppError::Internal("Config not found".into()))?;

    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".into()))?;

    let claims = validate_token(token, &config.jwt_secret)?;

    Ok(AuthenticatedUser {
        user_id: claims.sub,
    })
}

/// Extract user_id from a query parameter (used for WebSocket upgrade requests)
pub fn extract_user_from_query(query: &str, secret: &str) -> Result<String, AppError> {
    let token = query
        .split('&')
        .find_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            let key = parts.next()?;
            let value = parts.next()?;
            if key == "token" {
                Some(value.to_string())
            } else {
                None
            }
        })
        .ok_or_else(|| AppError::Unauthorized("Missing token query parameter".into()))?;

    let claims = validate_token(&token, secret)?;
    Ok(claims.sub)
}

/// Extract optional user_id from query parameter (supports guest mode)
pub fn extract_optional_user_from_query(query: &str, secret: &str) -> Option<String> {
    let token = query.split('&').find_map(|pair| {
        let mut parts = pair.splitn(2, '=');
        let key = parts.next()?;
        let value = parts.next()?;
        if key == "token" && !value.is_empty() {
            Some(value.to_string())
        } else {
            None
        }
    })?;

    validate_token(&token, secret).ok().map(|claims| claims.sub)
}
