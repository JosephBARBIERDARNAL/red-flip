use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub database_auth_token: Option<String>,
    pub jwt_secret: String,
    pub backend_port: u16,
    pub frontend_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            database_auth_token: env::var("DATABASE_AUTH_TOKEN")
                .ok()
                .filter(|s| !s.is_empty()),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            backend_port: env::var("BACKEND_PORT")
                .unwrap_or_else(|_| "8080".into())
                .parse()
                .expect("BACKEND_PORT must be a number"),
            frontend_url: env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:3000".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn from_env_reads_required_and_defaults() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());

        std::env::set_var("DATABASE_URL", "libsql://example.turso.io");
        std::env::remove_var("DATABASE_AUTH_TOKEN");
        std::env::set_var("JWT_SECRET", "test-secret");
        std::env::remove_var("BACKEND_PORT");
        std::env::remove_var("FRONTEND_URL");

        let cfg = AppConfig::from_env();

        assert_eq!(cfg.database_url, "libsql://example.turso.io");
        assert_eq!(cfg.database_auth_token, None);
        assert_eq!(cfg.jwt_secret, "test-secret");
        assert_eq!(cfg.backend_port, 8080);
        assert_eq!(cfg.frontend_url, "http://localhost:3000");
    }

    #[test]
    fn from_env_panics_without_database_url() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        std::env::remove_var("DATABASE_URL");
        std::env::set_var("JWT_SECRET", "test-secret");
        let result = std::panic::catch_unwind(AppConfig::from_env);
        assert!(result.is_err());
    }
}
