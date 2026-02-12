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
