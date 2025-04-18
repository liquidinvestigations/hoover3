//! Task definition macros, clients, workers - wrappers over Temporal SDK.

extern crate paste;
pub use inventory;
pub use paste::paste;

pub mod api;
mod client;
pub mod task_inventory;
pub mod tasks;
pub use client::*;
pub use tasks::*;

pub use hoover3_macro::{activity, workflow};

/// Environment variable for the worker tempdir (big space).
pub const WORKER_TEMPDIR_ENV_VAR_BIG: &str = "HOOVER3_WORKER_TEMP_DISK_BIG";
/// Environment variable for the worker tempdir (small space).
pub const WORKER_TEMPDIR_ENV_VAR_SMALL: &str = "HOOVER3_WORKER_TEMP_RAMDISK_SMALL";
