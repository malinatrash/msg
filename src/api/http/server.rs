use std::net::SocketAddr;

use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use super::router::create_router;
use super::state::AppState;
use crate::usecase::Service;

pub struct HttpServer {
    addr: SocketAddr,
    state: AppState,
}

impl HttpServer {
    pub fn new(host: String, port: u16, uc: Service) -> Self {
        let addr = format!("{}:{}", host, port)
            .parse()
            .expect("Invalid address");
        let state = AppState::new(uc);

        Self { addr, state }
    }

    pub async fn run(self) -> Result<(), String> {
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        let app = create_router(self.state)
            .layer(cors)
            .layer(TraceLayer::new_for_http());

        tracing::info!("HTTP server listening on {}", self.addr);

        let listener = tokio::net::TcpListener::bind(self.addr)
            .await
            .map_err(|e| format!("Failed to bind: {}", e))?;

        axum::serve(listener, app)
            .await
            .map_err(|e| format!("Server error: {}", e))
    }
}
