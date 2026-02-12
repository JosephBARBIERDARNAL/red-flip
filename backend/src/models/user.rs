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
        Ok(User {
            id: row
                .get::<String>(0)
                .map_err(|e| AppError::Internal(e.to_string()))?,
            username: row
                .get::<String>(1)
                .map_err(|e| AppError::Internal(e.to_string()))?,
            email: row
                .get::<String>(2)
                .map_err(|e| AppError::Internal(e.to_string()))?,
            password_hash: row
                .get::<Option<String>>(3)
                .map_err(|e| AppError::Internal(e.to_string()))?,
            avatar_url: row
                .get::<Option<String>>(5)
                .map_err(|e| AppError::Internal(e.to_string()))?,
            elo: row
                .get::<i32>(6)
                .map_err(|e| AppError::Internal(e.to_string()))?,
            total_games: row
                .get::<i32>(7)
                .map_err(|e| AppError::Internal(e.to_string()))?,
            wins: row
                .get::<i32>(8)
                .map_err(|e| AppError::Internal(e.to_string()))?,
            losses: row
                .get::<i32>(9)
                .map_err(|e| AppError::Internal(e.to_string()))?,
            draws: row
                .get::<i32>(10)
                .map_err(|e| AppError::Internal(e.to_string()))?,
            created_at: row
                .get::<String>(11)
                .map_err(|e| AppError::Internal(e.to_string()))?,
            updated_at: row
                .get::<String>(12)
                .map_err(|e| AppError::Internal(e.to_string()))?,
            is_banned: row
                .get::<i32>(13)
                .map_err(|e| AppError::Internal(e.to_string()))?
                != 0,
            banned_at: row
                .get::<Option<String>>(14)
                .map_err(|e| AppError::Internal(e.to_string()))?,
            banned_reason: row
                .get::<Option<String>>(15)
                .map_err(|e| AppError::Internal(e.to_string()))?,
            is_admin: row
                .get::<i32>(16)
                .map_err(|e| AppError::Internal(e.to_string()))?
                != 0,
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

    pub async fn create_from_google(
        db: &Database,
        username: &str,
        email: &str,
        google_id: &str,
        avatar_url: Option<&str>,
    ) -> Result<Self, AppError> {
        let id = Uuid::new_v4().to_string();
        let conn = db
            .connect()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "INSERT INTO users (id, username, email, google_id, avatar_url) VALUES (?1, ?2, ?3, ?4, ?5)",
            (id.clone(), username.to_string(), email.to_string(), google_id.to_string(), avatar_url.map(|s| s.to_string())),
        )
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

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

    pub async fn find_by_google_id(
        db: &Database,
        google_id: &str,
    ) -> Result<Option<Self>, AppError> {
        let conn = db
            .connect()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let mut rows = conn
            .query("SELECT * FROM users WHERE google_id = ?1", [google_id])
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

        if username.is_some() {
            updates.push("username = ?1".to_string());
        }
        if elo.is_some() {
            updates.push(format!("elo = ?{}", updates.len() + 1));
        }
        if wins.is_some() {
            updates.push(format!("wins = ?{}", updates.len() + 1));
        }
        if losses.is_some() {
            updates.push(format!("losses = ?{}", updates.len() + 1));
        }
        if draws.is_some() {
            updates.push(format!("draws = ?{}", updates.len() + 1));
        }

        if updates.is_empty() {
            return Ok(());
        }

        // Update total_games if any game stats are changed
        if wins.is_some() || losses.is_some() || draws.is_some() {
            updates.push("total_games = wins + losses + draws".to_string());
        }

        updates.push("updated_at = datetime('now')".to_string());
        query.push_str(&updates.join(", "));
        query.push_str(&format!(" WHERE id = ?{}", updates.len()));

        // Build the parameters tuple dynamically
        let mut params: Vec<String> = Vec::new();
        if let Some(u) = username {
            params.push(u.to_string());
        }
        if let Some(e) = elo {
            params.push(e.to_string());
        }
        if let Some(w) = wins {
            params.push(w.to_string());
        }
        if let Some(l) = losses {
            params.push(l.to_string());
        }
        if let Some(d) = draws {
            params.push(d.to_string());
        }
        params.push(user_id.to_string());

        // Execute with appropriate number of parameters
        let param_refs: Vec<&str> = params.iter().map(|s| s.as_str()).collect();
        conn.execute(&query, param_refs)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

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
}

#[derive(Debug, Serialize)]
pub struct PlatformStats {
    pub total_users: i64,
    pub active_users: i64,
    pub total_matches: i64,
    pub banned_users: i64,
}
