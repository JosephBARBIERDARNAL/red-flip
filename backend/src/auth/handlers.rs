use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::auth::jwt::create_token;
use crate::auth::middleware::AuthenticatedUser;
use crate::config::AppConfig;
use crate::db::Database;
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
    db: web::Data<Database>,
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
    let user = User::create(&db, &body.username, &body.email, &password_hash).await?;
    let token = create_token(&user.id, &config.jwt_secret)?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "token": token,
        "user": PublicUser::from(user),
    })))
}

pub async fn login(
    db: web::Data<Database>,
    config: web::Data<AppConfig>,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let user = User::find_by_email(&db, &body.email)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid email or password".into()))?;

    // Check if user is banned
    if user.is_banned {
        let reason = user
            .banned_reason
            .unwrap_or_else(|| "No reason provided".to_string());
        return Err(AppError::Unauthorized(format!(
            "Account banned: {}",
            reason
        )));
    }

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
    db: web::Data<Database>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let user = User::find_by_id(&db, &auth.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "user": PublicUser::from(user),
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::body::to_bytes;

    use crate::db::init_test_db;

    fn test_config() -> web::Data<AppConfig> {
        web::Data::new(AppConfig {
            database_url: "unused".into(),
            database_auth_token: None,
            jwt_secret: "test-secret".into(),
            backend_port: 8080,
            frontend_url: "http://localhost:3000".into(),
        })
    }

    #[actix_rt::test]
    async fn register_rejects_invalid_input() {
        let db = web::Data::new(init_test_db().await);
        let cfg = test_config();

        let short_username = register(
            db.clone(),
            cfg.clone(),
            web::Json(RegisterRequest {
                username: "ab".into(),
                email: "a@example.com".into(),
                password: "password".into(),
            }),
        )
        .await;
        assert!(matches!(short_username, Err(AppError::BadRequest(_))));

        let short_password = register(
            db,
            cfg,
            web::Json(RegisterRequest {
                username: "valid_name".into(),
                email: "b@example.com".into(),
                password: "123".into(),
            }),
        )
        .await;
        assert!(matches!(short_password, Err(AppError::BadRequest(_))));
    }

    #[actix_rt::test]
    async fn register_and_login_happy_path_and_banned_path() {
        let db = web::Data::new(init_test_db().await);
        let cfg = test_config();

        let register_response = register(
            db.clone(),
            cfg.clone(),
            web::Json(RegisterRequest {
                username: "auth_user".into(),
                email: "auth@example.com".into(),
                password: "secure-password".into(),
            }),
        )
        .await
        .expect("register should succeed");
        assert_eq!(register_response.status(), actix_web::http::StatusCode::CREATED);
        let reg_body = to_bytes(register_response.into_body())
            .await
            .expect("response body should be readable");
        let reg_json: serde_json::Value =
            serde_json::from_slice(&reg_body).expect("register response should be json");
        assert!(reg_json["token"].as_str().is_some());

        let login_response = login(
            db.clone(),
            cfg.clone(),
            web::Json(LoginRequest {
                email: "auth@example.com".into(),
                password: "secure-password".into(),
            }),
        )
        .await
        .expect("login should succeed");
        assert_eq!(login_response.status(), actix_web::http::StatusCode::OK);

        let created_user = User::find_by_email(&db, "auth@example.com")
            .await
            .expect("query should succeed")
            .expect("user should exist");
        User::ban(&db, &created_user.id, "rule violation")
            .await
            .expect("ban should succeed");

        let banned_login = login(
            db,
            cfg,
            web::Json(LoginRequest {
                email: "auth@example.com".into(),
                password: "secure-password".into(),
            }),
        )
        .await;
        assert!(matches!(banned_login, Err(AppError::Unauthorized(_))));
    }
}
