use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    Unauthorized(String),
    NotFound(String),
    Conflict(String),
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::BadRequest(msg) => write!(f, "Bad Request: {msg}"),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {msg}"),
            AppError::NotFound(msg) => write!(f, "Not Found: {msg}"),
            AppError::Conflict(msg) => write!(f, "Conflict: {msg}"),
            AppError::Internal(msg) => write!(f, "Internal Error: {msg}"),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, message) = match self {
            AppError::BadRequest(msg) => (actix_web::http::StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Unauthorized(msg) => (actix_web::http::StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::NotFound(msg) => (actix_web::http::StatusCode::NOT_FOUND, msg.clone()),
            AppError::Conflict(msg) => (actix_web::http::StatusCode::CONFLICT, msg.clone()),
            AppError::Internal(msg) => {
                log::error!("Internal error: {msg}");
                (
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".into(),
                )
            }
        };

        HttpResponse::build(status).json(serde_json::json!({ "error": message }))
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(err: bcrypt::BcryptError) -> Self {
        AppError::Internal(err.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        AppError::Unauthorized(err.to_string())
    }
}
