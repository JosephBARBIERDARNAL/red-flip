use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub backend_port: u16,
    pub frontend_url: String,
    pub google_client_id: Option<String>,
    pub google_client_secret: Option<String>,
    pub google_redirect_uri: Option<String>,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:red_flip.db".into()),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            backend_port: env::var("BACKEND_PORT")
                .unwrap_or_else(|_| "8080".into())
                .parse()
                .expect("BACKEND_PORT must be a number"),
            frontend_url: env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:3000".into()),
            google_client_id: env::var("GOOGLE_CLIENT_ID").ok().filter(|s| !s.is_empty()),
            google_client_secret: env::var("GOOGLE_CLIENT_SECRET")
                .ok()
                .filter(|s| !s.is_empty()),
            google_redirect_uri: env::var("GOOGLE_REDIRECT_URI")
                .ok()
                .filter(|s| !s.is_empty()),
        }
    }
}
