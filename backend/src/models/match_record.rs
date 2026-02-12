use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::errors::AppError;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MatchRecord {
    pub id: String,
    pub player1_id: String,
    pub player2_id: String,
    pub winner_id: Option<String>,
    pub is_ranked: bool,
    pub player1_score: i32,
    pub player2_score: i32,
    pub rounds_json: String,
    pub player1_elo_before: Option<i32>,
    pub player1_elo_after: Option<i32>,
    pub player2_elo_before: Option<i32>,
    pub player2_elo_after: Option<i32>,
    pub status: String,
    pub created_at: String,
    pub finished_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Round {
    pub round_number: i32,
    pub player1_choice: Option<String>,
    pub player2_choice: Option<String>,
    pub winner: Option<String>, // player1_id, player2_id, or "draw"
}

impl MatchRecord {
    pub async fn create(
        pool: &SqlitePool,
        player1_id: &str,
        player2_id: &str,
        is_ranked: bool,
        p1_elo: i32,
        p2_elo: i32,
    ) -> Result<Self, AppError> {
        let id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO matches (id, player1_id, player2_id, is_ranked, player1_elo_before, player2_elo_before) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(player1_id)
        .bind(player2_id)
        .bind(is_ranked)
        .bind(p1_elo)
        .bind(p2_elo)
        .execute(pool)
        .await?;

        Self::find_by_id(pool, &id)
            .await?
            .ok_or_else(|| AppError::Internal("Failed to fetch created match".into()))
    }

    pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Self>, AppError> {
        let m = sqlx::query_as::<_, Self>("SELECT * FROM matches WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(m)
    }

    pub async fn finish(
        pool: &SqlitePool,
        match_id: &str,
        winner_id: Option<&str>,
        p1_score: i32,
        p2_score: i32,
        rounds_json: &str,
        p1_elo_after: i32,
        p2_elo_after: i32,
        status: &str,
    ) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE matches SET winner_id = ?, player1_score = ?, player2_score = ?, rounds_json = ?, player1_elo_after = ?, player2_elo_after = ?, status = ?, finished_at = datetime('now') WHERE id = ?",
        )
        .bind(winner_id)
        .bind(p1_score)
        .bind(p2_score)
        .bind(rounds_json)
        .bind(p1_elo_after)
        .bind(p2_elo_after)
        .bind(status)
        .bind(match_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn recent_for_user(
        pool: &SqlitePool,
        user_id: &str,
        limit: i32,
    ) -> Result<Vec<Self>, AppError> {
        let matches = sqlx::query_as::<_, Self>(
            "SELECT * FROM matches WHERE (player1_id = ? OR player2_id = ?) AND status != 'in_progress' ORDER BY finished_at DESC LIMIT ?",
        )
        .bind(user_id)
        .bind(user_id)
        .bind(limit)
        .fetch_all(pool)
        .await?;
        Ok(matches)
    }
}
