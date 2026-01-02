use std::env;

#[derive(Debug)]
pub struct LoggerConfig {
    pub level: String,
}

impl LoggerConfig {
    pub fn new() -> Result<LoggerConfig, String> {
        let level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());

        Ok(LoggerConfig { level })
    }
}
