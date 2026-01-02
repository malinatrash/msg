use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init_telemetry(jaeger_endpoint: &str, service_name: &str) -> Result<(), String> {
    let provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(format!("http://{}", jaeger_endpoint)),
        )
        .with_trace_config(opentelemetry_sdk::trace::Config::default().with_resource(
            opentelemetry_sdk::Resource::new(vec![opentelemetry::KeyValue::new(
                "service.name",
                service_name.to_string(),
            )]),
        ))
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .map_err(|e| format!("Failed to initialize OpenTelemetry tracer: {}", e))?;

    let tracer = provider.tracer("msg-service");
    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_level(true);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(fmt_layer)
        .with(telemetry_layer)
        .init();

    tracing::info!(
        "OpenTelemetry initialized with Jaeger endpoint: {}",
        jaeger_endpoint
    );

    Ok(())
}

pub fn shutdown_telemetry() {
    opentelemetry::global::shutdown_tracer_provider();
}
