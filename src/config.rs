use std::env;
use std::fmt;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub env: String,
    pub host: String,
    pub port: u16,
    pub url: String,
    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let env = env::var("APP_ENV").unwrap_or_else(|_| "local".to_owned());
        let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());

        let port = match env::var("APP_PORT") {
            Ok(value) => value
                .parse::<u16>()
                .map_err(|_| ConfigError::InvalidPort(value))?,
            Err(_) => 3000,
        };

        let url = env::var("APP_URL").unwrap_or_else(|_| format!("http://{host}:{port}"));
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://rust_inertia_app:change-me-local@127.0.0.1:5432/rust_inertia_app".to_owned()
        });

        Ok(Self {
            env,
            host,
            port,
            url,
            database_url,
        })
    }

    pub fn secure_cookies(&self) -> bool {
        self.url.starts_with("https://")
    }
}

#[derive(Debug)]
pub enum ConfigError {
    InvalidPort(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPort(value) => write!(formatter, "APP_PORT is not a valid port: {value}"),
        }
    }
}

impl std::error::Error for ConfigError {}
