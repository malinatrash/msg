use std::env;

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: i64,
}

impl JwtConfig {
    pub fn new() -> Result<Self, String> {
        let secret =
            env::var("JWT_SECRET").map_err(|_| "JWT_SECRET environment variable not set")?;

        let expiration_hours = env::var("JWT_EXPIRATION_HOURS")
            .unwrap_or_else(|_| "24".to_string())
            .parse()
            .map_err(|_| "Invalid JWT_EXPIRATION_HOURS")?;

        Ok(Self {
            secret,
            expiration_hours,
        })
    }
}
