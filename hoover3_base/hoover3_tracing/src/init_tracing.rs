//! Tracing and logging initialization - used by both frontend and backend.

use dioxus_logger::tracing;
use tracing::Level;

use opentelemetry::sdk::Resource;
use opentelemetry::trace::TraceError;
use opentelemetry::{sdk::trace as sdktrace, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use tonic::metadata::MetadataMap;

/// Initialize tracing and log crates, using `dioxus_logger`
/// TODO: distributed tracing (SigNoz).
pub fn init_tracing() -> Result<sdktrace::Tracer, TraceError> {
    init_logging();

    tracing::info!("tracing init.");
    log::info!("log init.");
    let metadata = MetadataMap::new();

    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_metadata(metadata)
                .with_endpoint("http://localhost:4317"),
        )
        .with_trace_config(
            sdktrace::config().with_resource(Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                "hoover3",
            )])),
        )
        .install_batch(opentelemetry::runtime::Tokio)
}

fn init_logging() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    tracing_log::LogTracer::builder()
        .with_max_level(log::LevelFilter::Info)
        .init()
        .expect("failed to init logger x2");
}
