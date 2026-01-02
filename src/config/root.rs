use super::http::HttpConfig;
use super::jwt::JwtConfig;
use super::logger::LoggerConfig;
use super::postgres::PostgresConfig;
use super::telemetry::TelemetryConfig;

#[derive(Debug)]
pub struct Config {
    pub postgres: PostgresConfig,
    pub logger: LoggerConfig,
    pub jaeger: TelemetryConfig,
    pub jwt: JwtConfig,
    pub http: HttpConfig,
}

impl Config {
    pub fn new() -> Result<Config, String> {
        dotenv::dotenv().ok();

        let postgres = PostgresConfig::new()?;
        let logger = LoggerConfig::new()?;
        let jaeger = TelemetryConfig::new()?;
        let jwt = JwtConfig::new()?;
        let http = HttpConfig::new()?;

        Ok(Config {
            postgres,
            logger,
            jaeger,
            jwt,
            http,
        })
    }
}
