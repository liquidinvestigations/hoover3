//! Server API functions root - when no other module is appropriate

/// API method to get the current memory usage and limit for the server process, in MB
pub async fn get_server_memory_usage(_:()) -> anyhow::Result<(u32, u32)> {
    Ok((
        hoover3_tracing::get_process_memory_usage(),
        hoover3_tracing::get_process_memory_limit(),
    ))
}
