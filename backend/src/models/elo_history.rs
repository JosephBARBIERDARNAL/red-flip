use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::Database;
use crate::errors::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        db: &Database,
        user_id: &str,
        match_id: &str,
        elo_before: i32,
        elo_after: i32,
    ) -> Result<Self, AppError> {
        let id = Uuid::new_v4().to_string();
        let elo_change = elo_after - elo_before;
        let conn = db
            .connect()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "INSERT INTO elo_history (id, user_id, match_id, elo_before, elo_after, elo_change) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (id.clone(), user_id.to_string(), match_id.to_string(), elo_before, elo_after, elo_change),
        )
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_test_db;
    use crate::models::match_record::MatchRecord;
    use crate::models::user::User;

    #[actix_rt::test]
    async fn create_persists_elo_change() {
        let db = init_test_db().await;
        let p1 = User::create(&db, "elo_p1", "elo_p1@example.com", "hash")
            .await
            .expect("user should be created");
        let p2 = User::create(&db, "elo_p2", "elo_p2@example.com", "hash")
            .await
            .expect("user should be created");
        let m = MatchRecord::create(&db, &p1.id, &p2.id, true, 1000, 1000)
            .await
            .expect("match should be created");

        let history = EloHistory::create(&db, &p1.id, &m.id, 1000, 1018)
            .await
            .expect("history should be created");

        assert_eq!(history.elo_change, 18);
        assert_eq!(history.elo_before, 1000);
        assert_eq!(history.elo_after, 1018);

        let conn = db.connect().expect("connection should be available");
        let mut rows = conn
            .query(
                "SELECT COUNT(*) FROM elo_history WHERE id = ?1",
                [history.id.as_str()],
            )
            .await
            .expect("count query should succeed");
        let row = rows
            .next()
            .await
            .expect("row fetch should succeed")
            .expect("row should exist");
        let count: i64 = row.get(0).expect("count should be present");
        assert_eq!(count, 1);
    }
}
