use std::env;

#[derive(Debug)]
pub struct TelemetryConfig {
    pub endpoint: String,
    pub service_name: String,
}

impl TelemetryConfig {
    pub fn new() -> Result<TelemetryConfig, String> {
        let endpoint = env::var("OTEL_ENDPOINT")
            .map_err(|_| "OTEL_ENDPOINT environment variable not set".to_string())?;

        let service_name = env::var("OTEL_SERVICE_NAME")
            .map_err(|_| "OTEL_SERVICE_NAME environment variable not set".to_string())?;

        Ok(TelemetryConfig {
            endpoint,
            service_name,
        })
    }
}
