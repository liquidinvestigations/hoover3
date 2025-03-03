//! This module contains the table definitions for all the collection related models.

mod _declare_edge;
pub use _declare_edge::*;

mod _model_extra;
pub use _model_extra::*;

mod _scylla_graph_models;
pub use _scylla_graph_models::*;

mod _scylla_graph;
pub use _scylla_graph::*;

mod model_inventory;
pub use model_inventory::*;
