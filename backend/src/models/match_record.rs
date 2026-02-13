use libsql::Row;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::Database;
use crate::errors::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    fn from_row(row: &Row) -> Result<Self, AppError> {
        Ok(MatchRecord {
            id: row
                .get::<String>(0)
                .map_err(|e| AppError::Internal(e.to_string()))?,
            player1_id: row
                .get::<String>(1)
                .map_err(|e| AppError::Internal(e.to_string()))?,
            player2_id: row
                .get::<String>(2)
                .map_err(|e| AppError::Internal(e.to_string()))?,
            winner_id: row
                .get::<Option<String>>(3)
                .map_err(|e| AppError::Internal(e.to_string()))?,
            is_ranked: row
                .get::<i32>(4)
                .map_err(|e| AppError::Internal(e.to_string()))?
                != 0,
            player1_score: row
                .get::<i32>(5)
                .map_err(|e| AppError::Internal(e.to_string()))?,
            player2_score: row
                .get::<i32>(6)
                .map_err(|e| AppError::Internal(e.to_string()))?,
            rounds_json: row
                .get::<String>(7)
                .map_err(|e| AppError::Internal(e.to_string()))?,
            player1_elo_before: row
                .get::<Option<i32>>(8)
                .map_err(|e| AppError::Internal(e.to_string()))?,
            player1_elo_after: row
                .get::<Option<i32>>(9)
                .map_err(|e| AppError::Internal(e.to_string()))?,
            player2_elo_before: row
                .get::<Option<i32>>(10)
                .map_err(|e| AppError::Internal(e.to_string()))?,
            player2_elo_after: row
                .get::<Option<i32>>(11)
                .map_err(|e| AppError::Internal(e.to_string()))?,
            status: row
                .get::<String>(12)
                .map_err(|e| AppError::Internal(e.to_string()))?,
            created_at: row
                .get::<String>(13)
                .map_err(|e| AppError::Internal(e.to_string()))?,
            finished_at: row
                .get::<Option<String>>(14)
                .map_err(|e| AppError::Internal(e.to_string()))?,
        })
    }

    pub async fn create(
        db: &Database,
        player1_id: &str,
        player2_id: &str,
        is_ranked: bool,
        p1_elo: i32,
        p2_elo: i32,
    ) -> Result<Self, AppError> {
        let id = Uuid::new_v4().to_string();
        let conn = db
            .connect()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "INSERT INTO matches (id, player1_id, player2_id, is_ranked, player1_elo_before, player2_elo_before) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (id.clone(), player1_id.to_string(), player2_id.to_string(), is_ranked as i32, p1_elo, p2_elo),
        )
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        Self::find_by_id(db, &id)
            .await?
            .ok_or_else(|| AppError::Internal("Failed to fetch created match".into()))
    }

    pub async fn find_by_id(db: &Database, id: &str) -> Result<Option<Self>, AppError> {
        let conn = db
            .connect()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let mut rows = conn
            .query("SELECT * FROM matches WHERE id = ?1", [id])
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        match rows
            .next()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?
        {
            Some(row) => Ok(Some(Self::from_row(&row)?)),
            None => Ok(None),
        }
    }

    pub async fn finish(
        db: &Database,
        match_id: &str,
        winner_id: Option<&str>,
        p1_score: i32,
        p2_score: i32,
        rounds_json: &str,
        p1_elo_after: i32,
        p2_elo_after: i32,
        status: &str,
    ) -> Result<(), AppError> {
        let conn = db
            .connect()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "UPDATE matches SET winner_id = ?1, player1_score = ?2, player2_score = ?3, rounds_json = ?4, player1_elo_after = ?5, player2_elo_after = ?6, status = ?7, finished_at = datetime('now') WHERE id = ?8",
            (winner_id.map(|s| s.to_string()), p1_score, p2_score, rounds_json.to_string(), p1_elo_after, p2_elo_after, status.to_string(), match_id.to_string()),
        )
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(())
    }

    pub async fn recent_for_user(
        db: &Database,
        user_id: &str,
        limit: i32,
    ) -> Result<Vec<Self>, AppError> {
        let conn = db
            .connect()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let mut rows = conn
            .query(
                "SELECT * FROM matches WHERE (player1_id = ?1 OR player2_id = ?1) AND status != 'in_progress' ORDER BY finished_at DESC LIMIT ?2",
                [user_id, &limit.to_string()],
            )
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let mut matches = Vec::new();
        while let Some(row) = rows
            .next()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?
        {
            matches.push(Self::from_row(&row)?);
        }

        Ok(matches)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_test_db;
    use crate::models::user::User;

    async fn create_test_user(db: &Database, username: &str, email: &str) -> User {
        User::create(db, username, email, "hash")
            .await
            .expect("user should be created")
    }

    #[actix_rt::test]
    async fn create_finish_find_and_recent_for_user_work() {
        let db = init_test_db().await;
        let p1 = create_test_user(&db, "match_p1", "match_p1@example.com").await;
        let p2 = create_test_user(&db, "match_p2", "match_p2@example.com").await;

        let completed = MatchRecord::create(&db, &p1.id, &p2.id, true, 1000, 1000)
            .await
            .expect("match should be created");
        MatchRecord::finish(
            &db,
            &completed.id,
            Some(&p1.id),
            2,
            1,
            r#"[{"round_number":1}]"#,
            1016,
            984,
            "completed",
        )
        .await
        .expect("match should be finished");

        let _in_progress = MatchRecord::create(&db, &p1.id, &p2.id, false, 1016, 984)
            .await
            .expect("second match should be created");

        let found = MatchRecord::find_by_id(&db, &completed.id)
            .await
            .expect("query should succeed")
            .expect("match should exist");
        assert_eq!(found.winner_id.as_deref(), Some(p1.id.as_str()));
        assert_eq!(found.status, "completed");
        assert_eq!(found.player1_score, 2);
        assert_eq!(found.player2_score, 1);
        assert!(found.finished_at.is_some());

        let recent = MatchRecord::recent_for_user(&db, &p1.id, 10)
            .await
            .expect("recent query should succeed");
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].id, completed.id);
    }
}
