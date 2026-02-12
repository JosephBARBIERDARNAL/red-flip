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
    ];

    let conn = db.connect().expect("Failed to get connection");

    for migration in &migrations {
        // Split migration into individual statements (libsql doesn't support batches)
        let statements: Vec<&str> = migration
            .split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty() && !s.starts_with("--"))
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
