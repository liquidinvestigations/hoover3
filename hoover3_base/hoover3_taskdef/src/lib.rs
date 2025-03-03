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
