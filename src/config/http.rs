use std::env;

#[derive(Debug, Clone)]
pub struct HttpConfig {
    pub host: String,
    pub port: u16,
}

impl HttpConfig {
    pub fn new() -> Result<Self, String> {
        let host = env::var("HTTP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        let port = env::var("HTTP_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .map_err(|_| "Invalid HTTP_PORT")?;

        Ok(Self { host, port })
    }
}
