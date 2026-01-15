use serde::Deserialize;

const MIN_JWT_KEY_LEN: usize = 32;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub http_addr: String,
    pub grpc_addr: String,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration: i64,
    #[serde(default)]
    pub cors_origins: Vec<String>,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let http_port: u16 = std::env::var("HTTP_PORT")
            .unwrap_or_else(|_| "3000".into())
            .parse()
            .map_err(|e| anyhow::anyhow!("invalid HTTP_PORT: {}", e))?;
        let grpc_port: u16 = std::env::var("GRPC_PORT")
            .unwrap_or_else(|_| "50051".into())
            .parse()
            .map_err(|e| anyhow::anyhow!("invalid HTTP_PORT: {}", e))?;
        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| anyhow::anyhow!("DATABASE_URL must be set"))?;
        let jwt_secret = std::env::var("JWT_SECRET")
            .map_err(|_| anyhow::anyhow!("JWT_SECRET must be set"))
            .and_then(|secret| {
                if secret.len() >= MIN_JWT_KEY_LEN {
                    Ok(secret)
                } else {
                    Err(anyhow::anyhow!(
                        "JWT_SECRET must be at least {} characters long, got {}",
                        MIN_JWT_KEY_LEN,
                        secret.len()
                    ))
                }
            })?;
        let jwt_expiration = std::env::var("JWT_EXPIRATION")
            .unwrap_or_else(|_| "1440".into())
            .parse()
            .map_err(|e| anyhow::anyhow!("invalid JWT_EXPIRATION: {}", e))?;
        let cors_origins = std::env::var("CORS_ORIGINS")
            .unwrap_or_else(|_| "*".into())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(Self {
            http_addr: format!("{}:{}", host, http_port),
            grpc_addr: format!("{}:{}", host, grpc_port),
            database_url,
            jwt_secret,
            jwt_expiration,
            cors_origins,
        })
    }
}
