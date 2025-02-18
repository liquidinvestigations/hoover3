//! Implementation of the `#[activity]`, `#[workflow]`, `#[model]` macros.
//! If we implemented these in `hoover_macro` crate, a proc_macro crate, we couldn't unit test their internals.
//! So we have to implement them in a separate crate that's not a proc_macro crate.

pub use syn;

mod taskdef;
pub use taskdef::{activity, workflow};

mod models;
pub use models::model;
