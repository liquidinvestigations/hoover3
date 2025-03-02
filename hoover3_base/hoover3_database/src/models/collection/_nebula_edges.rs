//! This module contains the definitions for all the nebula edges.
//! Unit structs are used to identify edges, and are converted to `GraphEdgeType`s for use at runtime.

use hoover3_types::{db_schema::GraphEdgeType, identifier::DatabaseIdentifier};

/// Trait for unit structs that can be used to identify a Nebula edge.
/// These structs are to be used in code as a type safe identifier for an edge.
pub trait GraphEdgeIdentifier: Sized {
    /// Get the name of the edge.
    fn edge_name(&self) -> DatabaseIdentifier;
    /// Convert the edge to a `GraphEdgeType`.
    fn to_owned(&self) -> GraphEdgeType {
        GraphEdgeType {
            name: self.edge_name(),
        }
    }
}

impl GraphEdgeIdentifier for GraphEdgeType {
    fn edge_name(&self) -> DatabaseIdentifier {
        self.name.clone()
    }
}

macro_rules! declare_edge {
    ($id:ident, $ex:expr) => {
        /// Unit struct to identify a Nebula edge `$id``.
        pub struct $id;
        impl GraphEdgeIdentifier for $id {
            fn edge_name(&self) -> DatabaseIdentifier {
                DatabaseIdentifier::new($ex).expect("invalid edge name: is not DatabaseIdentifier")
            }
        }
    };
}

declare_edge!(FilesystemParentEdge, "filesystem_parent");

/// Get all the Nebula edge types.
/// Don't forget to add new edges here...
pub fn get_all_nebula_edge_types() -> Vec<GraphEdgeType> {
    vec![FilesystemParentEdge.to_owned()]
}
