//! Task definition macros, clients, workers - wrappers over Temporal SDK.

extern crate paste;
pub use paste::paste;

mod client;
pub mod tasks;

pub use client::*;
pub use tasks::*;

pub use hoover3_macro::{activity, workflow};
