//! This module contains the definitions for all the nebula edges.
//! Unit structs are used to identify edges, and are converted to `GraphEdgeType`s for use at runtime.

use charybdis::model::BaseModel;
use hoover3_types::{db_schema::GraphEdgeType, identifier::CollectionId};

use super::EdgeBatchOperation;

/// Trait for unit structs that can be used to identify a Nebula edge.
/// These structs are to be used in code as a type safe identifier for an edge.
pub trait GraphEdge: Sized {
    type SourceType: BaseModel + Send + Sync + 'static;
    type DestType: BaseModel + Send + Sync + 'static;

    /// Get the name of the edge type.
    fn edge_type() -> GraphEdgeType;

    /// Get a batch for inserting into this edge type.
    fn edge_batch(collection_id: &CollectionId) -> EdgeBatchOperation<Self> {
        EdgeBatchOperation::<Self>::new(collection_id.clone())
    }
}

#[macro_export]
macro_rules! declare_graph_edge {
    ($struct_name:ident, $edge_name:expr, $source:ty, $dest:ty) => {
        /// Unit struct to identify a graph edge `$id``.
        pub struct $struct_name;
        impl $crate::models::collection::GraphEdge for $struct_name {
            fn edge_type() -> ::hoover3_types::db_schema::GraphEdgeType {
                ::hoover3_types::db_schema::GraphEdgeType {
                    edge_type: ::hoover3_types::identifier::DatabaseIdentifier::new($edge_name)
                        .expect("invalid edge name: is not DatabaseIdentifier"),
                }
            }
            type SourceType = $source;
            type DestType = $dest;
        }
        ::hoover3_types::inventory::submit!(::hoover3_types::db_schema::GraphEdgeTypeStatic {
            edge_type: $edge_name,
            source_type: $crate::paste::paste!(
                <$source as $crate::charybdis::model::BaseModel>::DB_MODEL_NAME
            ),
            target_type: $crate::paste::paste!(
                <$dest as $crate::charybdis::model::BaseModel>::DB_MODEL_NAME
            ),
        });
    };
}
pub use declare_graph_edge;

#[cfg(test)]
mod test {
    use super::*;
    use charybdis::macros::charybdis_model;
    use charybdis::types::Text;

    #[charybdis_model(
        table_name = test_graph_edge,
        partition_keys = [id],
        clustering_keys = [],
    )]
    pub struct TestModel {
        pub id: Text,
    }

    declare_graph_edge!(TestEdge, "graph_test_edge", TestModel, TestModel);

    /// this test checks that macro compiles ok
    #[test]
    fn test_declare_graph_edge_macro_compiles() {
        let mut _batch = TestEdge::edge_batch(&CollectionId::new("test_batch_collection").unwrap());
        _batch.add_edge(
            &TestModel {
                id: "test_source".into(),
            },
            &TestModel {
                id: "test_dest".into(),
            },
        );
    }
}
