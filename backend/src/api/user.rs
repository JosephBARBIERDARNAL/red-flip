use actix_web::{web, HttpResponse};

use crate::auth::middleware::AuthenticatedUser;
use crate::db::Database;
use crate::errors::AppError;
use crate::models::user::{PublicUser, User};

pub async fn get_user(
    db: web::Data<Database>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
    let user = User::find_by_id(&db, &user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "user": PublicUser::from(user),
    })))
}

pub async fn delete_account(
    db: web::Data<Database>,
    auth_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    // Verify the user exists before deleting
    User::find_by_id(&db, &auth_user.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    // Delete the user and all related data
    User::delete(&db, &auth_user.user_id).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Account deleted successfully",
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_test_db;

    #[actix_rt::test]
    async fn get_user_returns_not_found_for_missing_user() {
        let db = web::Data::new(init_test_db().await);
        let result = get_user(db, web::Path::from("missing-id".to_string())).await;
        assert!(matches!(result, Err(AppError::NotFound(_))));
    }

    #[actix_rt::test]
    async fn delete_account_deletes_existing_user() {
        let db = web::Data::new(init_test_db().await);
        let created = User::create(&db, "delete_me", "deleteme@example.com", "hash")
            .await
            .expect("user should be created");

        let resp = delete_account(
            db.clone(),
            AuthenticatedUser {
                user_id: created.id.clone(),
            },
        )
        .await
        .expect("delete account should succeed");

        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
        let user = User::find_by_id(&db, &created.id)
            .await
            .expect("query should succeed");
        assert!(user.is_none());
    }
}
