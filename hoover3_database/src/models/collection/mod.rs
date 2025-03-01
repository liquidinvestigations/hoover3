//! This module contains the table definitions for all the collection related models.

pub mod _nebula_edges;

mod _model_extra;
pub use _model_extra::*;

mod _scylla_graph_models;
pub use _scylla_graph_models::*;

mod _scylla_graph;
pub use _scylla_graph::*;
