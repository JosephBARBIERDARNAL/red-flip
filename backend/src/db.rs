use libsql::Builder;
use std::sync::Arc;

pub type Database = Arc<libsql::Database>;

pub async fn init_pool(database_url: &str, auth_token: Option<&str>) -> Database {
    let db = if database_url.starts_with("libsql://") || database_url.starts_with("https://") {
        // Remote Turso database
        let token = auth_token.expect("AUTH_TOKEN required for remote database");
        Builder::new_remote(database_url.to_string(), token.to_string())
            .build()
            .await
            .expect("Failed to connect to remote database")
    } else {
        // Local SQLite file
        Builder::new_local(database_url.strip_prefix("sqlite:").unwrap_or(database_url))
            .build()
            .await
            .expect("Failed to create local database")
    };

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
            conn.execute(statement, ())
                .await
                .expect(&format!("Failed to run migration statement: {}", statement));
        }
    }

    log::info!("Database migrations completed");
}
