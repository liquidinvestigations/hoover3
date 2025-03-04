//! Tracing and logging initialization - used by both frontend and backend.

use dioxus_logger::tracing;
use tracing::Level;

/// Initialize tracing and log crates, using `dioxus_logger`
/// TODO: distributed tracing (SigNoz).
pub fn init_tracing() {
    init_logging();

    tracing::info!("tracing init.");
    log::info!("log init.");
}

fn init_logging() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    tracing_log::LogTracer::builder()
        .with_max_level(log::LevelFilter::Info)
        .init()
        .expect("failed to init logger x2");
}
