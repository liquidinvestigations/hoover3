//! This module contains the definitions for all the graph edges.
//! Unit structs are used to identify edges, and are converted to `GraphEdgeType`s for use at runtime.

use std::future::Future;

use charybdis::model::BaseModel;
use hoover3_types::{db_schema::GraphEdgeType, identifier::CollectionId};
use serde::{Deserialize, Serialize};

use super::{
    query_edge::{graph_edge_sources_for_target, graph_edge_targets_for_source, ResultStream},
    EdgeBatchOperation,
};

/// Trait for unit structs that can be used to identify a graph edge.
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

    /// Go over edge in the forward direction
    /// from a source node, and return a stream of all the target nodes.
    fn graph_edge_targets_for_source(
        collection_id: &CollectionId,
        source: &<Self::SourceType as BaseModel>::PrimaryKey,
    ) -> impl Future<Output = anyhow::Result<ResultStream<<Self::DestType as BaseModel>::PrimaryKey>>>
    where
        <Self::SourceType as BaseModel>::PrimaryKey:
            Send + Sync + 'static + Serialize + for<'a> Deserialize<'a>,
        <Self::DestType as BaseModel>::PrimaryKey:
            Send + Sync + 'static + Serialize + for<'a> Deserialize<'a>,
    {
        graph_edge_targets_for_source::<Self>(collection_id, source)
    }

    /// Go over edge in the reverse direction
    /// from a target node, and return a stream of all the source nodes.
    fn graph_edge_sources_for_target(
        collection_id: &CollectionId,
        target: &<Self::DestType as BaseModel>::PrimaryKey,
    ) -> impl Future<Output = anyhow::Result<ResultStream<<Self::SourceType as BaseModel>::PrimaryKey>>>
    where
        <Self::SourceType as BaseModel>::PrimaryKey:
            Send + Sync + 'static + Serialize + for<'a> Deserialize<'a>,
        <Self::DestType as BaseModel>::PrimaryKey:
            Send + Sync + 'static + Serialize + for<'a> Deserialize<'a>,
    {
        graph_edge_sources_for_target::<Self>(collection_id, target)
    }
}

#[macro_export]
macro_rules! declare_graph_edge {
    ($struct_name:ident, $edge_name:expr, $source:ty, $dest:ty) => {
        /// Unit struct to identify a graph edge `$id``.
        pub struct $struct_name;
        impl $crate::models::collection::GraphEdge for $struct_name {
            fn edge_type() -> ::hoover3_types::db_schema::GraphEdgeType {
                ::hoover3_types::db_schema::GraphEdgeType(
                    ::hoover3_types::identifier::DatabaseIdentifier::new($edge_name)
                        .expect("invalid edge name: is not DatabaseIdentifier"),
                )
            }
            type SourceType = $source;
            type DestType = $dest;
        }
        $crate::inventory::submit!($crate::models::collection::GraphEdgeTypeStatic {
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
