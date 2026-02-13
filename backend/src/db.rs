use libsql::Builder;
use std::sync::Arc;

pub type Database = Arc<libsql::Database>;

pub async fn init_pool(database_url: &str, auth_token: Option<&str>) -> Database {
    let token = auth_token.expect("AUTH_TOKEN required for remote database");
    let db = Builder::new_remote(database_url.to_string(), token.to_string())
        .build()
        .await
        .expect("Failed to connect to remote database");

    Arc::new(db)
}

pub async fn run_migrations(db: &Database) {
    let migrations = [
        include_str!("../migrations/001_create_users.sql"),
        include_str!("../migrations/002_create_matches.sql"),
        include_str!("../migrations/003_create_elo_history.sql"),
        include_str!("../migrations/004_add_admin_fields.sql"),
        include_str!("../migrations/005_add_ai_players.sql"),
    ];

    let conn = db.connect().expect("Failed to get connection");

    for migration in &migrations {
        // Remove comment-only lines, then split into individual statements
        // (libsql doesn't support batch execution).
        let sanitized_migration = migration
            .lines()
            .filter(|line| !line.trim_start().starts_with("--"))
            .collect::<Vec<_>>()
            .join("\n");

        let statements: Vec<&str> = sanitized_migration
            .split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        for statement in statements {
            match conn.execute(statement, ()).await {
                Ok(_) => {}
                Err(e) => {
                    let err_str = e.to_string();
                    // Ignore errors for already existing tables/columns/indexes (idempotent migrations)
                    if err_str.contains("already exists") || err_str.contains("duplicate column") {
                        log::debug!("Skipping migration (already applied): {}", statement);
                    } else {
                        panic!(
                            "Failed to run migration statement: {}\nError: {}",
                            statement, err_str
                        );
                    }
                }
            }
        }
    }

    log::info!("Database migrations completed");
}

#[cfg(test)]
pub async fn init_test_db() -> Database {
    let path = std::env::temp_dir().join(format!("red_flip_test_{}.db", uuid::Uuid::new_v4()));
    let db = Builder::new_local(path)
        .build()
        .await
        .expect("Failed to create local test database");
    let db = Arc::new(db);
    run_migrations(&db).await;
    db
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn run_migrations_is_idempotent() {
        let db = init_test_db().await;
        run_migrations(&db).await;

        let conn = db.connect().expect("connection should be available");
        let mut rows = conn
            .query(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'users'",
                [] as [&str; 0],
            )
            .await
            .expect("query should succeed");

        let row = rows
            .next()
            .await
            .expect("row fetch should succeed")
            .expect("row should exist");
        let count: i64 = row.get(0).expect("count column should exist");
        assert_eq!(count, 1);
    }
}
