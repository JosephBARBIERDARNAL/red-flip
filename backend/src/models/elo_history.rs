use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::errors::AppError;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EloHistory {
    pub id: String,
    pub user_id: String,
    pub match_id: String,
    pub elo_before: i32,
    pub elo_after: i32,
    pub elo_change: i32,
    pub created_at: String,
}

impl EloHistory {
    pub async fn create(
        pool: &SqlitePool,
        user_id: &str,
        match_id: &str,
        elo_before: i32,
        elo_after: i32,
    ) -> Result<Self, AppError> {
        let id = Uuid::new_v4().to_string();
        let elo_change = elo_after - elo_before;
        sqlx::query(
            "INSERT INTO elo_history (id, user_id, match_id, elo_before, elo_after, elo_change) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(user_id)
        .bind(match_id)
        .bind(elo_before)
        .bind(elo_after)
        .bind(elo_change)
        .execute(pool)
        .await?;

        Ok(Self {
            id,
            user_id: user_id.to_string(),
            match_id: match_id.to_string(),
            elo_before,
            elo_after,
            elo_change,
            created_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        })
    }
}
