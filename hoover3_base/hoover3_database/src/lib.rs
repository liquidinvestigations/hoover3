//! This module contains the database clients, model definitions, and various management functions.
//! It also hosts a bunch of client methods for interacting with the various databases.

pub mod client_query;
pub mod db_management;
pub mod migrate;
pub mod models;
pub mod system_paths;

// re-export for usage in macros:
pub use charybdis;
pub use paste;
