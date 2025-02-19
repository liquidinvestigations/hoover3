//! Procedural macros for Hoover3 workflow engine.
//!
//! # Task Definitions
//!
//! This crate provides the `#[activity]` and `#[workflow]` attribute macros that are used to
//! define activities and workflows in Hoover3.
//!
//! ### Activities
//!
//! Activities are the basic unit of work in Hoover3. They can be either synchronous or asynchronous:
//!
//! ```rust
//! #[activity("my_queue")]
//! async fn my_activity(input: MyInput) -> anyhow::Result<MyOutput> {
//!     // Activity implementation
//! }
//! ```
//!
//! ### Workflows
//!
//! Workflows orchestrate activities and must be asynchronous:
//!
//! ```rust
//! #[workflow("my_queue")]
//! async fn my_workflow(ctx: WfContext, input: MyInput) -> WorkflowResult<MyOutput> {
//!     // Workflow implementation
//! }
//! ```
//!
//! # Models
//!
//! This crate provides the `#[model]` macro that is appended to a
//! struct definition to declare database tables and edges, as well as mark field options.
//!
//! ---
//!
//! #  Implementation
//!
//! The implementation is in the hoover3_macro2 crate.

use proc_macro::TokenStream;

/// Attribute macro for defining activities. Its only argument is the queue name.
#[proc_macro_attribute]
pub fn activity(_attr: TokenStream, item: TokenStream) -> TokenStream {
    hoover3_macro2::activity(_attr.into(), item.into()).into()
}

/// Attribute macro for defining workflows. Its only argument is the queue name.
#[proc_macro_attribute]
pub fn workflow(_attr: TokenStream, item: TokenStream) -> TokenStream {
    hoover3_macro2::workflow(_attr.into(), item.into()).into()
}

/// Attribute macro for defining model.
#[proc_macro_attribute]
pub fn model(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    hoover3_macro2::model(item.into()).into()
}

/// Attribute macro for defining UDT model.
#[proc_macro_attribute]
pub fn udt_model(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    hoover3_macro2::udt_model(item.into()).into()
}
