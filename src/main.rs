use msg_service::bootstrap::App;

#[tokio::main]
async fn main() {
    match App::new() {
        Ok(app) => {
            if let Err(e) = app.run().await {
                tracing::error!("Application error: {}", e);
                msg_service::bootstrap::telemetry::shutdown_telemetry();
                std::process::exit(1);
            }
            msg_service::bootstrap::telemetry::shutdown_telemetry();
        }
        Err(e) => {
            eprintln!("Failed to initialize application: {}", e);
            std::process::exit(1);
        }
    }
}
