//! Tracing and logging initialization - used by both frontend and backend.

use dioxus_logger::tracing;
use tracing::Level;

use opentelemetry::global;
use opentelemetry::trace::TracerProvider;
use opentelemetry_sdk::runtime::Tokio;
use opentelemetry_sdk::trace::Config;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

/// Initialize tracing and log crates, using `dioxus_logger`
/// TODO: distributed tracing (SigNoz).
pub fn init_tracing() {
    init_logging();

    tracing::info!("tracing init.");
    log::info!("log init.");

    let provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .with_trace_config(Config::default())
        .install_batch(Tokio)
        .expect("failed to install opentelemetry");

    let tracer = provider.tracer_builder("opentelemetry-otlp").build();
    global::set_tracer_provider(provider.clone());
    let telemetry = OpenTelemetryLayer::new(tracer.clone());

    let subscriber = Registry::default().with(telemetry);

    tracing::subscriber::set_global_default(subscriber).expect("setting global default failed");
}

fn init_logging() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    tracing_log::LogTracer::builder()
        .with_max_level(log::LevelFilter::Info)
        .init()
        .expect("failed to init logger x2");
}
