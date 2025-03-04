use crate::ALLOCATOR;

/// Set the memory limit for the current process.
pub fn set_process_memory_limit(limit_mb: u32) -> anyhow::Result<()> {
    tracing::info!("setting process memory limit to {} MB", limit_mb);
    let limit = limit_mb as u64 * 1024 * 1024;
    // on wasm32 we have usize = 32bit so max alloc is 2GB
    let limit = limit.min(usize::MAX as u64) as usize;
    ALLOCATOR
        .set_limit(limit)
        .map_err(|e| anyhow::anyhow!("set memory limit error: {:?}", e))?;
    Ok(())
}

/// Get the current memory limit for the current process in MB.
pub fn get_process_memory_limit() -> u32 {
    (ALLOCATOR.limit() / 1024 / 1024)  as u32
}

/// Get the current memory usage for the current process in MB.
pub fn get_process_memory_usage() -> u32 {
    (ALLOCATOR.allocated()  / 1024 / 1024) as u32
}
