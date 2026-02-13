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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::jwt::create_token;

    #[test]
    fn extract_optional_user_from_query_returns_user_id_for_valid_token() {
        let secret = "test-secret";
        let token = create_token("user-42", secret).expect("token should be created");

        let user_id =
            extract_optional_user_from_query(&format!("foo=bar&token={token}"), secret);

        assert_eq!(user_id.as_deref(), Some("user-42"));
    }

    #[test]
    fn extract_optional_user_from_query_returns_none_for_missing_or_invalid_token() {
        let secret = "test-secret";
        let invalid = extract_optional_user_from_query("foo=bar", secret);
        assert!(invalid.is_none());

        let wrong_secret_token =
            create_token("user-42", "different-secret").expect("token should be created");
        let invalid =
            extract_optional_user_from_query(&format!("token={wrong_secret_token}"), secret);
        assert!(invalid.is_none());
    }
}
