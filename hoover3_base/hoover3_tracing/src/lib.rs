//! Tracing and logging initialization - used by both frontend and backend.

pub use init_tracing::*;
mod init_tracing;

mod memory_limit;
pub use memory_limit::*;

/// Globally limit memory, defaulting to 2 GB
#[global_allocator]
pub static ALLOCATOR: cap::Cap<std::alloc::System> =
    cap::Cap::new(std::alloc::System, 2 * 1024 * 1024 * 1024);

pub use tracing;

#[cfg(feature = "telemetry")]
pub mod telemetry;
