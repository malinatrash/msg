use std::env;

#[derive(Debug)]
pub struct GrpcConfig {
    pub port: u16,
}

impl GrpcConfig {
    pub fn new() -> Result<GrpcConfig, String> {
        let port = env::var("GRPC_PORT")
            .map_err(|_| "GRPC_PORT environment variable not set".to_string())?
            .parse::<u16>()
            .map_err(|_| "Invalid GRPC_PORT value".to_string())?;

        Ok(GrpcConfig { port })
    }
}
