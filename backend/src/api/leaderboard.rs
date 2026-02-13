use actix_web::{web, HttpResponse};

use crate::db::Database;
use crate::errors::AppError;
use crate::models::user::{PublicUser, User};

pub async fn get_leaderboard(db: web::Data<Database>) -> Result<HttpResponse, AppError> {
    let users = User::top_by_elo(&db, 10).await?;
    let public_users: Vec<PublicUser> = users.into_iter().map(PublicUser::from).collect();

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "leaderboard": public_users,
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::body::to_bytes;

    use crate::db::init_test_db;

    #[actix_rt::test]
    async fn leaderboard_endpoint_returns_payload() {
        let db = web::Data::new(init_test_db().await);
        let resp = get_leaderboard(db).await.expect("leaderboard should succeed");

        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
        let body = to_bytes(resp.into_body())
            .await
            .expect("response body should be readable");
        let json: serde_json::Value =
            serde_json::from_slice(&body).expect("response should be valid json");
        assert!(json.get("leaderboard").is_some());
    }
}
