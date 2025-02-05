//! This module contains time helpers for WASM clients.

/// WASM-compatible: Get the current time in seconds since the Unix epoch.
pub fn current_time() -> f64 {
    web_time::SystemTime::now()
        .duration_since(web_time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

/// WASM-compatible: Sleep for a given duration.
pub async fn sleep(duration: std::time::Duration) {
    async_std::task::sleep(duration).await;
}
