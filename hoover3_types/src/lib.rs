//! Protocol consisting of types and structures shared across frontend and backend.

pub mod collection;
pub mod datasource;
pub mod db_schema;
pub mod docker_health;
pub mod filesystem;
pub mod identifier;
pub mod stable_hash;
pub mod tasks;

/// re-export for usage in macros
pub use inventory;

// Globally limit memory to 2 GB
#[global_allocator]
static ALLOCATOR: cap::Cap<std::alloc::System> =
    cap::Cap::new(std::alloc::System, 2 * 1024 * 1024 * 1024);
