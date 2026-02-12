use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::errors::AppError;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
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
    pub async fn create(
        pool: &SqlitePool,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<Self, AppError> {
        let id = Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO users (id, username, email, password_hash) VALUES (?, ?, ?, ?)")
            .bind(&id)
            .bind(username)
            .bind(email)
            .bind(password_hash)
            .execute(pool)
            .await
            .map_err(|e| {
                if e.to_string().contains("UNIQUE") {
                    if e.to_string().contains("username") {
                        AppError::Conflict("Username already taken".into())
                    } else {
                        AppError::Conflict("Email already registered".into())
                    }
                } else {
                    AppError::Internal(e.to_string())
                }
            })?;

        Self::find_by_id(pool, &id)
            .await?
            .ok_or_else(|| AppError::Internal("Failed to fetch created user".into()))
    }

    pub async fn create_from_google(
        pool: &SqlitePool,
        username: &str,
        email: &str,
        google_id: &str,
        avatar_url: Option<&str>,
    ) -> Result<Self, AppError> {
        let id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO users (id, username, email, google_id, avatar_url) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(username)
        .bind(email)
        .bind(google_id)
        .bind(avatar_url)
        .execute(pool)
        .await?;

        Self::find_by_id(pool, &id)
            .await?
            .ok_or_else(|| AppError::Internal("Failed to fetch created user".into()))
    }

    pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Self>, AppError> {
        let user = sqlx::query_as::<_, Self>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(user)
    }

    pub async fn find_by_email(pool: &SqlitePool, email: &str) -> Result<Option<Self>, AppError> {
        let user = sqlx::query_as::<_, Self>("SELECT * FROM users WHERE email = ?")
            .bind(email)
            .fetch_optional(pool)
            .await?;
        Ok(user)
    }

    pub async fn find_by_google_id(
        pool: &SqlitePool,
        google_id: &str,
    ) -> Result<Option<Self>, AppError> {
        let user = sqlx::query_as::<_, Self>("SELECT * FROM users WHERE google_id = ?")
            .bind(google_id)
            .fetch_optional(pool)
            .await?;
        Ok(user)
    }

    pub async fn update_elo(
        pool: &SqlitePool,
        user_id: &str,
        new_elo: i32,
    ) -> Result<(), AppError> {
        sqlx::query("UPDATE users SET elo = ?, updated_at = datetime('now') WHERE id = ?")
            .bind(new_elo)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn increment_stats(
        pool: &SqlitePool,
        user_id: &str,
        won: Option<bool>,
    ) -> Result<(), AppError> {
        let query = match won {
            Some(true) => {
                "UPDATE users SET total_games = total_games + 1, wins = wins + 1, updated_at = datetime('now') WHERE id = ?"
            }
            Some(false) => {
                "UPDATE users SET total_games = total_games + 1, losses = losses + 1, updated_at = datetime('now') WHERE id = ?"
            }
            None => {
                "UPDATE users SET total_games = total_games + 1, draws = draws + 1, updated_at = datetime('now') WHERE id = ?"
            }
        };
        sqlx::query(query).bind(user_id).execute(pool).await?;
        Ok(())
    }

    pub async fn top_by_elo(pool: &SqlitePool, limit: i32) -> Result<Vec<Self>, AppError> {
        let users = sqlx::query_as::<_, Self>("SELECT * FROM users ORDER BY elo DESC LIMIT ?")
            .bind(limit)
            .fetch_all(pool)
            .await?;
        Ok(users)
    }

    pub async fn delete(pool: &SqlitePool, user_id: &str) -> Result<(), AppError> {
        // Delete related records first (cascade)
        sqlx::query("DELETE FROM elo_history WHERE user_id = ?")
            .bind(user_id)
            .execute(pool)
            .await?;

        sqlx::query("DELETE FROM matches WHERE player1_id = ? OR player2_id = ?")
            .bind(user_id)
            .bind(user_id)
            .execute(pool)
            .await?;

        // Finally delete the user
        sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    // Admin methods
    pub async fn is_admin(pool: &SqlitePool, user_id: &str) -> Result<bool, AppError> {
        let result = sqlx::query_scalar::<_, bool>("SELECT is_admin FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_optional(pool)
            .await?;
        Ok(result.unwrap_or(false))
    }

    pub async fn list_with_filters(
        pool: &SqlitePool,
        search: Option<&str>,
        sort_by: Option<&str>,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<Self>, AppError> {
        let mut query = String::from("SELECT * FROM users WHERE 1=1");

        if let Some(s) = search {
            if !s.is_empty() {
                query.push_str(" AND (username LIKE '%' || ? || '%' OR email LIKE '%' || ? || '%')");
            }
        }

        let order = match sort_by {
            Some("elo") => "elo DESC",
            Some("created_at") => "created_at DESC",
            Some("total_games") => "total_games DESC",
            _ => "created_at DESC",
        };

        query.push_str(&format!(" ORDER BY {} LIMIT ? OFFSET ?", order));

        let mut q = sqlx::query_as::<_, Self>(&query);

        if let Some(s) = search {
            if !s.is_empty() {
                q = q.bind(s).bind(s);
            }
        }

        let users = q.bind(limit).bind(offset).fetch_all(pool).await?;
        Ok(users)
    }

    pub async fn count_all(pool: &SqlitePool, search: Option<&str>) -> Result<i64, AppError> {
        let mut query = String::from("SELECT COUNT(*) FROM users WHERE 1=1");

        if let Some(s) = search {
            if !s.is_empty() {
                query.push_str(" AND (username LIKE '%' || ? || '%' OR email LIKE '%' || ? || '%')");
            }
        }

        let mut q = sqlx::query_scalar::<_, i64>(&query);

        if let Some(s) = search {
            if !s.is_empty() {
                q = q.bind(s).bind(s);
            }
        }

        let count = q.fetch_one(pool).await?;
        Ok(count)
    }

    pub async fn ban(
        pool: &SqlitePool,
        user_id: &str,
        reason: &str,
    ) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE users SET is_banned = 1, banned_at = datetime('now'), banned_reason = ?, updated_at = datetime('now') WHERE id = ?"
        )
        .bind(reason)
        .bind(user_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn unban(pool: &SqlitePool, user_id: &str) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE users SET is_banned = 0, banned_at = NULL, banned_reason = NULL, updated_at = datetime('now') WHERE id = ?"
        )
        .bind(user_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn update_stats(
        pool: &SqlitePool,
        user_id: &str,
        username: Option<&str>,
        elo: Option<i32>,
        wins: Option<i32>,
        losses: Option<i32>,
        draws: Option<i32>,
    ) -> Result<(), AppError> {
        let mut query = String::from("UPDATE users SET ");
        let mut updates = Vec::new();
        let mut bindings: Vec<String> = Vec::new();

        if let Some(u) = username {
            updates.push("username = ?");
            bindings.push(u.to_string());
        }
        if let Some(e) = elo {
            updates.push("elo = ?");
            bindings.push(e.to_string());
        }
        if let Some(w) = wins {
            updates.push("wins = ?");
            bindings.push(w.to_string());
        }
        if let Some(l) = losses {
            updates.push("losses = ?");
            bindings.push(l.to_string());
        }
        if let Some(d) = draws {
            updates.push("draws = ?");
            bindings.push(d.to_string());
        }

        if updates.is_empty() {
            return Ok(());
        }

        // Update total_games if any game stats are changed
        if wins.is_some() || losses.is_some() || draws.is_some() {
            updates.push("total_games = wins + losses + draws");
        }

        updates.push("updated_at = datetime('now')");
        query.push_str(&updates.join(", "));
        query.push_str(" WHERE id = ?");
        bindings.push(user_id.to_string());

        let mut q = sqlx::query(&query);
        for binding in bindings {
            q = q.bind(binding);
        }

        q.execute(pool).await?;
        Ok(())
    }

    pub async fn get_platform_stats(pool: &SqlitePool) -> Result<PlatformStats, AppError> {
        let total_users = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
            .fetch_one(pool)
            .await?;

        let active_users = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(DISTINCT user_id) FROM elo_history WHERE created_at > datetime('now', '-30 days')"
        )
        .fetch_one(pool)
        .await?;

        let total_matches = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM matches")
            .fetch_one(pool)
            .await?;

        let banned_users = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE is_banned = 1")
            .fetch_one(pool)
            .await?;

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
