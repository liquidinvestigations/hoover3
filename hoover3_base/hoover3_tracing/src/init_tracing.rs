//! Tracing and logging initialization - used by both frontend and backend.
use dioxus_logger::tracing;
use tracing::info;
use tracing::Level;
use log::info as log_info;
#[cfg(feature = "telemetry")]
use opentelemetry_sdk::logs::SdkLoggerProvider;
#[cfg(feature = "telemetry")]
use opentelemetry_sdk::trace::SdkTracerProvider;

#[cfg(feature = "telemetry")]
type TelemetryReturn = Option<(SdkLoggerProvider, SdkTracerProvider)>;

#[cfg(not(feature = "telemetry"))]
type TelemetryReturn = ();

/// Initialize tracing and log crates, using `dioxus_logger`
/// TODO: distributed tracing (SigNoz).
pub async fn init_tracing() -> TelemetryReturn {
    #[cfg(not(feature = "telemetry"))]
{
    init_logging();
    info!("tracing init.");
    log_info!("log init.");
    }



    #[cfg(feature = "telemetry")]
{
    let (logger_provider, tracer_provider) = crate::telemetry::init_telemetry().await;
    info!("telemetry init finished.");
    Some((logger_provider, tracer_provider))
    }
}

fn init_logging() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    tracing_log::LogTracer::builder()
        .with_max_level(log::LevelFilter::Info)
        .init()
        .expect("failed to init logger x2");
}
