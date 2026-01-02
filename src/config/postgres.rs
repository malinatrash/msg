use std::env;

#[derive(Debug)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl PostgresConfig {
    pub fn new() -> Result<PostgresConfig, String> {
        let host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());

        let port = env::var("POSTGRES_PORT")
            .unwrap_or_else(|_| "5432".to_string())
            .parse::<u16>()
            .map_err(|_| "Invalid POSTGRES_PORT value".to_string())?;

        let user = env::var("POSTGRES_USER")
            .map_err(|_| "POSTGRES_USER environment variable not set".to_string())?;

        let password = env::var("POSTGRES_PASSWORD")
            .map_err(|_| "POSTGRES_PASSWORD environment variable not set".to_string())?;

        let database = env::var("POSTGRES_DATABASE")
            .map_err(|_| "POSTGRES_DATABASE environment variable not set".to_string())?;

        Ok(PostgresConfig {
            host,
            port,
            user,
            password,
            database,
        })
    }
}
