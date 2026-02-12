use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::str::FromStr;

pub async fn init_pool(database_url: &str) -> SqlitePool {
    let options = SqliteConnectOptions::from_str(database_url)
        .expect("Invalid DATABASE_URL")
        .create_if_missing(true);

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .expect("Failed to create database pool")
}

pub async fn run_migrations(pool: &SqlitePool) {
    let migrations = [
        include_str!("../migrations/001_create_users.sql"),
        include_str!("../migrations/002_create_matches.sql"),
        include_str!("../migrations/003_create_elo_history.sql"),
    ];

    for migration in &migrations {
        sqlx::query(migration)
            .execute(pool)
            .await
            .expect("Failed to run migration");
    }

    log::info!("Database migrations completed");
}
