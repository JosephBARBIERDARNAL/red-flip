use libsql::Row;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::Database;
use crate::errors::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,
    #[serde(skip_serializing)]
    pub avatar_url: Option<String>,
    pub elo: i32,
    pub total_games: i32,
    pub wins: i32,
    pub losses: i32,
    pub draws: i32,
    pub created_at: String,
    pub updated_at: String,
    pub is_admin: bool,
    pub is_banned: bool,
    pub banned_at: Option<String>,
    pub banned_reason: Option<String>,
    pub is_ai: bool,
}

#[derive(Debug, Serialize)]
pub struct PublicUser {
    pub id: String,
    pub username: String,
    pub avatar_url: Option<String>,
    pub elo: i32,
    pub total_games: i32,
    pub wins: i32,
    pub losses: i32,
    pub draws: i32,
    pub is_admin: bool,
}

impl From<User> for PublicUser {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            username: u.username,
            avatar_url: u.avatar_url,
            elo: u.elo,
            total_games: u.total_games,
            wins: u.wins,
            losses: u.losses,
            draws: u.draws,
            is_admin: u.is_admin,
        }
    }
}

impl User {
    fn from_row(row: &Row) -> Result<Self, AppError> {
        let column_index = |name: &str| -> Option<i32> {
            (0..row.column_count()).find(|&i| row.column_name(i) == Some(name))
        };

        let get_required = |name: &str| -> Result<String, AppError> {
            let idx = column_index(name)
                .ok_or_else(|| AppError::Internal(format!("Missing `{name}` column")))?;
            row.get::<String>(idx)
                .map_err(|e| AppError::Internal(e.to_string()))
        };

        let get_optional = |name: &str| -> Result<Option<String>, AppError> {
            match column_index(name) {
                Some(idx) => row
                    .get::<Option<String>>(idx)
                    .map_err(|e| AppError::Internal(e.to_string())),
                None => Ok(None),
            }
        };

        let get_i32 = |name: &str, default: i32| -> Result<i32, AppError> {
            match column_index(name) {
                Some(idx) => row
                    .get::<Option<i32>>(idx)
                    .map_err(|e| AppError::Internal(e.to_string()))
                    .map(|v| v.unwrap_or(default)),
                None => Ok(default),
            }
        };

        let get_bool = |name: &str, default: bool| -> Result<bool, AppError> {
            get_i32(name, if default { 1 } else { 0 }).map(|v| v != 0)
        };

        Ok(User {
            id: get_required("id")?,
            username: get_required("username")?,
            email: get_required("email")?,
            password_hash: get_optional("password_hash")?,
            avatar_url: get_optional("avatar_url")?,
            elo: get_i32("elo", 1000)?,
            total_games: get_i32("total_games", 0)?,
            wins: get_i32("wins", 0)?,
            losses: get_i32("losses", 0)?,
            draws: get_i32("draws", 0)?,
            created_at: get_required("created_at")?,
            updated_at: get_required("updated_at")?,
            is_admin: get_bool("is_admin", false)?,
            is_banned: get_bool("is_banned", false)?,
            banned_at: get_optional("banned_at")?,
            banned_reason: get_optional("banned_reason")?,
            is_ai: get_bool("is_ai", false)?,
        })
    }

    pub async fn create(
        db: &Database,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<Self, AppError> {
        let id = Uuid::new_v4().to_string();
        let conn = db
            .connect()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "INSERT INTO users (id, username, email, password_hash) VALUES (?1, ?2, ?3, ?4)",
            (
                id.clone(),
                username.to_string(),
                email.to_string(),
                password_hash.to_string(),
            ),
        )
        .await
        .map_err(|e| {
            let err_str = e.to_string();
            if err_str.contains("UNIQUE") {
                if err_str.contains("username") {
                    AppError::Conflict("Username already taken".into())
                } else {
                    AppError::Conflict("Email already registered".into())
                }
            } else {
                AppError::Internal(err_str)
            }
        })?;

        Self::find_by_id(db, &id)
            .await?
            .ok_or_else(|| AppError::Internal("Failed to fetch created user".into()))
    }

    pub async fn find_by_id(db: &Database, id: &str) -> Result<Option<Self>, AppError> {
        let conn = db
            .connect()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let mut rows = conn
            .query("SELECT * FROM users WHERE id = ?1", [id])
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

    pub async fn find_by_email(db: &Database, email: &str) -> Result<Option<Self>, AppError> {
        let conn = db
            .connect()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let mut rows = conn
            .query("SELECT * FROM users WHERE email = ?1", [email])
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

    pub async fn update_elo(db: &Database, user_id: &str, new_elo: i32) -> Result<(), AppError> {
        let conn = db
            .connect()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "UPDATE users SET elo = ?1, updated_at = datetime('now') WHERE id = ?2",
            (new_elo, user_id.to_string()),
        )
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(())
    }

    pub async fn increment_stats(
        db: &Database,
        user_id: &str,
        won: Option<bool>,
    ) -> Result<(), AppError> {
        let conn = db
            .connect()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let query = match won {
            Some(true) => {
                "UPDATE users SET total_games = total_games + 1, wins = wins + 1, updated_at = datetime('now') WHERE id = ?1"
            }
            Some(false) => {
                "UPDATE users SET total_games = total_games + 1, losses = losses + 1, updated_at = datetime('now') WHERE id = ?1"
            }
            None => {
                "UPDATE users SET total_games = total_games + 1, draws = draws + 1, updated_at = datetime('now') WHERE id = ?1"
            }
        };

        conn.execute(query, [user_id])
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(())
    }

    pub async fn top_by_elo(db: &Database, limit: i32) -> Result<Vec<Self>, AppError> {
        let conn = db
            .connect()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let mut rows = conn
            .query("SELECT * FROM users ORDER BY elo DESC LIMIT ?1", [limit])
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let mut users = Vec::new();
        while let Some(row) = rows
            .next()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?
        {
            users.push(Self::from_row(&row)?);
        }

        Ok(users)
    }

    pub async fn delete(db: &Database, user_id: &str) -> Result<(), AppError> {
        let conn = db
            .connect()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        // Delete related records first (cascade)
        conn.execute("DELETE FROM elo_history WHERE user_id = ?1", [user_id])
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "DELETE FROM matches WHERE player1_id = ?1 OR player2_id = ?1",
            [user_id],
        )
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        // Finally delete the user
        conn.execute("DELETE FROM users WHERE id = ?1", [user_id])
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(())
    }

    // Admin methods
    pub async fn is_admin(db: &Database, user_id: &str) -> Result<bool, AppError> {
        let conn = db
            .connect()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let mut rows = conn
            .query("SELECT is_admin FROM users WHERE id = ?1", [user_id])
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        match rows
            .next()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?
        {
            Some(row) => {
                let is_admin: i32 = row.get(0).map_err(|e| AppError::Internal(e.to_string()))?;
                Ok(is_admin != 0)
            }
            None => Ok(false),
        }
    }

    pub async fn list_with_filters(
        db: &Database,
        search: Option<&str>,
        sort_by: Option<&str>,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<Self>, AppError> {
        let conn = db
            .connect()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let mut query = String::from("SELECT * FROM users WHERE 1=1");
        let mut params: Vec<String> = Vec::new();

        if let Some(s) = search {
            if !s.is_empty() {
                query.push_str(
                    " AND (username LIKE '%' || ?1 || '%' OR email LIKE '%' || ?1 || '%')",
                );
                params.push(s.to_string());
            }
        }

        let order = match sort_by {
            Some("elo") => "elo DESC",
            Some("created_at") => "created_at DESC",
            Some("total_games") => "total_games DESC",
            _ => "created_at DESC",
        };

        query.push_str(&format!(
            " ORDER BY {} LIMIT ?{} OFFSET ?{}",
            order,
            params.len() + 1,
            params.len() + 2
        ));
        params.push(limit.to_string());
        params.push(offset.to_string());

        let mut rows = if params.len() == 2 {
            conn.query(&query, [params[0].as_str(), params[1].as_str()])
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?
        } else {
            conn.query(
                &query,
                [params[0].as_str(), params[1].as_str(), params[2].as_str()],
            )
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?
        };

        let mut users = Vec::new();
        while let Some(row) = rows
            .next()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?
        {
            users.push(Self::from_row(&row)?);
        }

        Ok(users)
    }

    pub async fn count_all(db: &Database, search: Option<&str>) -> Result<i64, AppError> {
        let conn = db
            .connect()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let mut query = String::from("SELECT COUNT(*) FROM users WHERE 1=1");

        let mut rows = if let Some(s) = search {
            if !s.is_empty() {
                query.push_str(
                    " AND (username LIKE '%' || ?1 || '%' OR email LIKE '%' || ?1 || '%')",
                );
                conn.query(&query, [s])
                    .await
                    .map_err(|e| AppError::Internal(e.to_string()))?
            } else {
                conn.query(&query, [] as [&str; 0])
                    .await
                    .map_err(|e| AppError::Internal(e.to_string()))?
            }
        } else {
            conn.query(&query, [] as [&str; 0])
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?
        };

        match rows
            .next()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?
        {
            Some(row) => {
                let count: i64 = row.get(0).map_err(|e| AppError::Internal(e.to_string()))?;
                Ok(count)
            }
            None => Ok(0),
        }
    }

    pub async fn ban(db: &Database, user_id: &str, reason: &str) -> Result<(), AppError> {
        let conn = db
            .connect()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "UPDATE users SET is_banned = 1, banned_at = datetime('now'), banned_reason = ?1, updated_at = datetime('now') WHERE id = ?2",
            (reason.to_string(), user_id.to_string()),
        )
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(())
    }

    pub async fn unban(db: &Database, user_id: &str) -> Result<(), AppError> {
        let conn = db
            .connect()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "UPDATE users SET is_banned = 0, banned_at = NULL, banned_reason = NULL, updated_at = datetime('now') WHERE id = ?1",
            [user_id],
        )
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(())
    }

    pub async fn update_stats(
        db: &Database,
        user_id: &str,
        username: Option<&str>,
        elo: Option<i32>,
        wins: Option<i32>,
        losses: Option<i32>,
        draws: Option<i32>,
    ) -> Result<(), AppError> {
        let conn = db
            .connect()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let mut query = String::from("UPDATE users SET ");
        let mut updates: Vec<String> = Vec::new();
        let mut params: Vec<String> = Vec::new();

        if let Some(u) = username {
            params.push(u.to_string());
            updates.push(format!("username = ?{}", params.len()));
        }
        if let Some(e) = elo {
            params.push(e.to_string());
            updates.push(format!("elo = ?{}", params.len()));
        }
        if let Some(w) = wins {
            params.push(w.to_string());
            updates.push(format!("wins = ?{}", params.len()));
        }
        if let Some(l) = losses {
            params.push(l.to_string());
            updates.push(format!("losses = ?{}", params.len()));
        }
        if let Some(d) = draws {
            params.push(d.to_string());
            updates.push(format!("draws = ?{}", params.len()));
        }

        if updates.is_empty() {
            return Ok(());
        }

        let should_refresh_total_games = wins.is_some() || losses.is_some() || draws.is_some();

        updates.push("updated_at = datetime('now')".to_string());
        query.push_str(&updates.join(", "));
        params.push(user_id.to_string());
        query.push_str(&format!(" WHERE id = ?{}", params.len()));

        // Execute with appropriate number of parameters
        let param_refs: Vec<&str> = params.iter().map(|s| s.as_str()).collect();
        conn.execute(&query, param_refs)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        if should_refresh_total_games {
            conn.execute(
                "UPDATE users SET total_games = wins + losses + draws, updated_at = datetime('now') WHERE id = ?1",
                [user_id],
            )
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        }

        Ok(())
    }

    pub async fn get_platform_stats(db: &Database) -> Result<PlatformStats, AppError> {
        let conn = db
            .connect()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let mut rows = conn
            .query("SELECT COUNT(*) FROM users", [] as [&str; 0])
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let total_users: i64 = rows
            .next()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?
            .ok_or_else(|| AppError::Internal("Failed to get total users".into()))?
            .get(0)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let mut rows = conn.query(
            "SELECT COUNT(DISTINCT user_id) FROM elo_history WHERE created_at > datetime('now', '-30 days')",
            [] as [&str; 0],
        ).await.map_err(|e| AppError::Internal(e.to_string()))?;
        let active_users: i64 = rows
            .next()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?
            .ok_or_else(|| AppError::Internal("Failed to get active users".into()))?
            .get(0)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let mut rows = conn
            .query("SELECT COUNT(*) FROM matches", [] as [&str; 0])
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let total_matches: i64 = rows
            .next()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?
            .ok_or_else(|| AppError::Internal("Failed to get total matches".into()))?
            .get(0)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let mut rows = conn
            .query(
                "SELECT COUNT(*) FROM users WHERE is_banned = 1",
                [] as [&str; 0],
            )
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let banned_users: i64 = rows
            .next()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?
            .ok_or_else(|| AppError::Internal("Failed to get banned users".into()))?
            .get(0)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(PlatformStats {
            total_users,
            active_users,
            total_matches,
            banned_users,
        })
    }

    pub async fn get_random_ai(db: &Database) -> Result<Self, AppError> {
        let conn = db
            .connect()
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let mut rows = conn
            .query(
                "SELECT * FROM users WHERE is_ai = 1 ORDER BY RANDOM() LIMIT 1",
                [] as [&str; 0],
            )
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        match rows
            .next()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?
        {
            Some(row) => Ok(Self::from_row(&row)?),
            None => Err(AppError::Internal("No AI users available".into())),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PlatformStats {
    pub total_users: i64,
    pub active_users: i64,
    pub total_matches: i64,
    pub banned_users: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_test_db;
    use crate::errors::AppError;

    async fn create_test_user(db: &Database, username: &str, email: &str) -> User {
        User::create(db, username, email, "hashed-password")
            .await
            .expect("user should be created")
    }

    #[actix_rt::test]
    async fn create_and_find_user_with_defaults() {
        let db = init_test_db().await;

        let created = create_test_user(&db, "alice_test", "alice@example.com").await;
        let by_id = User::find_by_id(&db, &created.id)
            .await
            .expect("query should succeed")
            .expect("user should exist");
        let by_email = User::find_by_email(&db, "alice@example.com")
            .await
            .expect("query should succeed")
            .expect("user should exist");

        assert_eq!(by_id.id, created.id);
        assert_eq!(by_email.username, "alice_test");
        assert_eq!(by_id.elo, 1000);
        assert_eq!(by_id.total_games, 0);
        assert!(!by_id.is_admin);
        assert!(!by_id.is_ai);
    }

    #[actix_rt::test]
    async fn create_enforces_unique_username_and_email() {
        let db = init_test_db().await;
        let _ = create_test_user(&db, "duplicate_name", "first@example.com").await;

        let duplicate_username =
            User::create(&db, "duplicate_name", "second@example.com", "hash").await;
        assert!(matches!(duplicate_username, Err(AppError::Conflict(_))));

        let duplicate_email = User::create(&db, "other_name", "first@example.com", "hash").await;
        assert!(matches!(duplicate_email, Err(AppError::Conflict(_))));
    }

    #[actix_rt::test]
    async fn update_elo_and_stats_paths_work() {
        let db = init_test_db().await;
        let user = create_test_user(&db, "stats_user", "stats@example.com").await;

        User::update_elo(&db, &user.id, 1450)
            .await
            .expect("elo update should succeed");
        User::increment_stats(&db, &user.id, Some(true))
            .await
            .expect("win increment should succeed");
        User::increment_stats(&db, &user.id, Some(false))
            .await
            .expect("loss increment should succeed");
        User::increment_stats(&db, &user.id, None)
            .await
            .expect("draw increment should succeed");

        User::update_stats(
            &db,
            &user.id,
            Some("stats_user_renamed"),
            Some(1500),
            Some(4),
            Some(2),
            Some(1),
        )
        .await
        .expect("stats update should succeed");

        let updated = User::find_by_id(&db, &user.id)
            .await
            .expect("query should succeed")
            .expect("user should exist");

        assert_eq!(updated.username, "stats_user_renamed");
        assert_eq!(updated.elo, 1500);
        assert_eq!(updated.wins, 4);
        assert_eq!(updated.losses, 2);
        assert_eq!(updated.draws, 1);
        assert_eq!(updated.total_games, 7);
    }

    #[actix_rt::test]
    async fn admin_ban_unban_filter_and_count_work() {
        let db = init_test_db().await;
        let admin = create_test_user(&db, "admin_user", "admin@example.com").await;
        let target = create_test_user(&db, "target_user", "target@example.com").await;

        let conn = db.connect().expect("connection should be available");
        conn.execute(
            "UPDATE users SET is_admin = 1 WHERE id = ?1",
            [admin.id.as_str()],
        )
        .await
        .expect("admin update should succeed");

        assert!(User::is_admin(&db, &admin.id)
            .await
            .expect("admin check should succeed"));
        assert!(!User::is_admin(&db, &target.id)
            .await
            .expect("admin check should succeed"));

        User::ban(&db, &target.id, "test reason")
            .await
            .expect("ban should succeed");
        let banned = User::find_by_id(&db, &target.id)
            .await
            .expect("query should succeed")
            .expect("user should exist");
        assert!(banned.is_banned);
        assert_eq!(banned.banned_reason.as_deref(), Some("test reason"));

        User::unban(&db, &target.id)
            .await
            .expect("unban should succeed");
        let unbanned = User::find_by_id(&db, &target.id)
            .await
            .expect("query should succeed")
            .expect("user should exist");
        assert!(!unbanned.is_banned);
        assert!(unbanned.banned_reason.is_none());

        User::update_elo(&db, &admin.id, 2000)
            .await
            .expect("elo update should succeed");
        User::update_elo(&db, &target.id, 1800)
            .await
            .expect("elo update should succeed");

        let users = User::list_with_filters(&db, Some("user"), Some("elo"), 0, 5)
            .await
            .expect("list query should succeed");
        assert!(!users.is_empty());
        assert!(users.windows(2).all(|w| w[0].elo >= w[1].elo));

        let filtered_count = User::count_all(&db, Some("example.com"))
            .await
            .expect("count query should succeed");
        assert!(filtered_count >= 2);
    }

    #[actix_rt::test]
    async fn platform_stats_and_random_ai_are_available() {
        let db = init_test_db().await;
        let _ = create_test_user(&db, "normal_user", "normal@example.com").await;

        let stats = User::get_platform_stats(&db)
            .await
            .expect("platform stats should load");
        assert!(stats.total_users >= 101);
        assert_eq!(stats.banned_users, 0);

        let ai = User::get_random_ai(&db)
            .await
            .expect("an AI user should exist");
        assert!(ai.is_ai);
    }
}
