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
}
