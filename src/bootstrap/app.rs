use std::sync::Arc;

use super::logger::Logger;
use super::postgres::Postgres;
use crate::api::http::HttpServer;
use crate::config::Config;
use crate::repository::Repository;
use crate::usecase::Service;

pub struct App {
    pub config: Arc<Config>,
    pub logger: Arc<Logger>,
    pub postgres: Arc<Postgres>,
    pub repo: Repository,
    pub uc: Service,
}

impl App {
    pub fn new() -> Result<Self, String> {
        let config = Arc::new(Config::new()?);

        super::telemetry::init_telemetry(&config.jaeger.endpoint, &config.jaeger.service_name)?;

        let logger = Logger::new(&config.logger);
        let postgres = Postgres::new(&config.postgres)?;
        let repo = Repository::new(postgres.clone());
        let uc = Service::new(repo.clone(), config.jwt.clone());

        tracing::info!("Application initialized successfully");

        Ok(App {
            config,
            logger,
            postgres,
            repo,
            uc,
        })
    }

    pub async fn run(self) -> Result<(), String> {
        tracing::info!("Application starting...");
        tracing::debug!("Config: {:?}", self.config);

        let http_server = HttpServer::new(
            self.config.http.host.clone(),
            self.config.http.port,
            self.uc.clone(),
        );

        http_server.run().await?;

        Ok(())
    }
}
