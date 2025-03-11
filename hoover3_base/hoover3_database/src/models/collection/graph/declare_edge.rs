//! This module contains the definitions for all the graph edges.
//! Unit structs are used to identify edges, and are converted to `GraphEdgeType`s for use at runtime.

use async_stream::try_stream;
use futures::pin_mut;
use futures::FutureExt;
use futures::StreamExt;
use futures::TryStreamExt;
use hoover3_types::identifier::DatabaseIdentifier;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use charybdis::{model::BaseModel, operations::Find};
use hoover3_types::{db_schema::GraphEdgeId, identifier::CollectionId};
use serde::{Deserialize, Serialize};

use crate::db_management::{DatabaseSpaceManager, ScyllaDatabaseHandle};

use super::{query_edge::ResultStream, EdgeBatchOperation};

/// Helper trait to ensure that the key value is serializable and deserializable.
/// Auto-implemented for all compatible types.
/// This will mostly be applied to BaseModel::PrimaryKey and BaseModel::PartitionKey,
/// which are tuples of strings and numbers and booleans.
pub trait PKValue:
    Serialize + for<'a> Deserialize<'a> + Send + Sync + 'static + Clone + Default
{
}
impl<T: Serialize + for<'a> Deserialize<'a> + Send + Sync + 'static + Clone + Default> PKValue
    for T
{
}

/// Helper trait to ensure that the primary key is serializable and deserializable.
/// Auto-implemented for all models that implement `BaseModel`.
pub trait BaseModel2: BaseModel + Send + Sync + 'static
where
    <Self as BaseModel>::PrimaryKey: PKValue,
    <Self as BaseModel>::PartitionKey: PKValue,
{
    fn primary_to_partition(primary: &Self::PrimaryKey) -> Self::PartitionKey;
}

impl<T: BaseModel> BaseModel2 for T
where
    T: BaseModel + Send + Sync + 'static,
    <T as BaseModel>::PrimaryKey: PKValue,
    <T as BaseModel>::PartitionKey: PKValue,
{
    /// Helper function to convert a primary key to a partition key.
    /// This should be implemented in a macro, but it's not, so we hack it with json
    fn primary_to_partition(
        primary: &<Self as BaseModel>::PrimaryKey,
    ) -> <Self as BaseModel>::PartitionKey {
        let primary_json = serde_json::to_value(primary).unwrap();
        let partition_default = serde_json::to_value(&Self::PartitionKey::default()).unwrap();

        let primary_vec = primary_json.as_array().unwrap().to_vec();
        let mut partition_vec = partition_default.as_array().unwrap().to_vec();

        if primary_vec.len() == partition_vec.len() {
            return serde_json::from_value(primary_json).unwrap();
        }
        for (primary_val, partition_val) in primary_vec.into_iter().zip(partition_vec.iter_mut()) {
            *partition_val = primary_val;
        }
        serde_json::from_value(serde_json::Value::Array(partition_vec)).unwrap()
    }
}

/// Trait for unit structs that can be used to identify a graph edge.
/// These structs are to be used in code as a type safe identifier for an edge.
pub trait GraphEdge: Send + Sync + 'static
where
    <Self::SourceType as BaseModel>::PrimaryKey: PKValue,
    <Self::DestType as BaseModel>::PrimaryKey: PKValue,
    <Self::SourceType as BaseModel>::PartitionKey: PKValue,
    <Self::DestType as BaseModel>::PartitionKey: PKValue,
{
    /// Get the name of the edge type.
    fn edge_type() -> GraphEdgeId
    where
        Self: Sized;
    type SourceType: BaseModel2;
    type DestType: BaseModel2;
}

/// Trait for graph edges that can be queried.
pub trait GraphEdgeQuery: GraphEdge {
    /// Go over edge in the forward direction
    /// from a source node, and return a stream of all the target node PKs.
    fn list_target(
        &self,
        collection_id: &CollectionId,
        source: &<Self::SourceType as BaseModel>::PrimaryKey,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<ResultStream<Self::DestType>>> + Send>>;

    /// Go over edge in the reverse direction
    /// from a target node, and return a stream of all the source node PKs.
    fn list_source(
        &self,
        collection_id: &CollectionId,
        target: &<Self::DestType as BaseModel>::PrimaryKey,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<ResultStream<Self::SourceType>>> + Send>>;
}

/// Trait for graph edges that can be inserted into the database.
pub trait GraphEdgeInsert: GraphEdge {
    /// Get a batch for inserting into this edge type.
    fn edge_batch(collection_id: &CollectionId) -> EdgeBatchOperation<Self>
    where
        Self: Sized;
}

// https://docs.rs/type-equals/latest/src/type_equals/lib.rs.html#99-101
pub trait TypeEquals {
    type Other: ?Sized;
}
impl<T: ?Sized> TypeEquals for T {
    type Other = Self;
}

pub trait ParentChildRelationship: GraphEdge {
    fn child_partition_to_parent_primary(
        child_partition: &<Self::DestType as BaseModel>::PartitionKey,
    ) -> <Self::SourceType as BaseModel>::PrimaryKey;

    fn parent_primary_to_child_partition(
        parent_primary: &<Self::SourceType as BaseModel>::PrimaryKey,
    ) -> <Self::DestType as BaseModel>::PartitionKey;
}

impl<T> GraphEdgeQuery for T
where
    T: ParentChildRelationship,
    T: GraphEdge,
{
    fn list_target(
        &self,
        collection_id: &CollectionId,
        source: &<Self::SourceType as BaseModel>::PrimaryKey,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<ResultStream<Self::DestType>>> + Send>> {
        let target_partition = T::parent_primary_to_child_partition(source);
        let collection_id = collection_id.clone();
        async move {
            let session = ScyllaDatabaseHandle::collection_session(&collection_id).await?;
            let stream =
                <<T as GraphEdge>::DestType as Find>::find_by_partition_key_value(target_partition)
                    .execute(&session)
                    .await?;
            let stream =
                stream.map_err(|e| anyhow::anyhow!(format!("Error listing target pks: {:?}", e)));
            anyhow::Ok(stream.boxed())
        }
        .boxed()
    }

    fn list_source(
        &self,
        collection_id: &CollectionId,
        target: &<Self::DestType as BaseModel>::PrimaryKey,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<ResultStream<Self::SourceType>>> + Send>> {
        let child_partition = <Self::DestType as BaseModel2>::primary_to_partition(target);
        let collection_id = collection_id.clone();
        let parent_primary = T::child_partition_to_parent_primary(&child_partition);
        async move {
            let session = ScyllaDatabaseHandle::collection_session(&collection_id).await?;
            let row = <T::SourceType as Find>::find_by_primary_key_value(parent_primary)
                .execute(&session)
                .await?;
            let stream = futures::stream::once(async move { Ok(row) });
            anyhow::Ok(stream.boxed())
        }
        .boxed()
    }
}

/// Trait for implicit graph edges, where the partition key of the child is the primary key of the parent.
pub trait GraphEdgeImplicit: GraphEdge
where
    <Self::SourceType as BaseModel>::PrimaryKey:
        TypeEquals<Other = <Self::DestType as BaseModel>::PartitionKey>,
{
}
impl<T: GraphEdgeImplicit> ParentChildRelationship for T
where
    <Self::SourceType as BaseModel>::PrimaryKey:
        TypeEquals<Other = <Self::DestType as BaseModel>::PartitionKey>,
{
    fn child_partition_to_parent_primary(
        child_primary: &<Self::DestType as BaseModel>::PartitionKey,
    ) -> <Self::SourceType as BaseModel>::PrimaryKey {
        let v = child_primary as &dyn std::any::Any;
        v.downcast_ref::<<Self::SourceType as BaseModel>::PrimaryKey>()
            .unwrap()
            .clone()
    }

    fn parent_primary_to_child_partition(
        parent_primary: &<Self::SourceType as BaseModel>::PrimaryKey,
    ) -> <Self::DestType as BaseModel>::PartitionKey {
        let v = parent_primary as &dyn std::any::Any;
        v.downcast_ref::<<Self::DestType as BaseModel>::PartitionKey>()
            .unwrap()
            .clone()
    }
}

#[macro_export]
macro_rules! declare_stored_graph_edge {
    ($struct_name:ident, $edge_name:expr, $source:ty, $dest:ty) => {
        $crate::paste::paste! {

            /// Unit struct to identify a stored graph edge `$id``.
            #[derive(Debug, Clone, Copy)]
            pub struct $struct_name;

            $crate::inventory::submit!($crate::models::collection::GraphEdgeTypeStatic {
                edge_type: $edge_name,
                source_type: <$source as  $crate::charybdis::model::BaseModel>::DB_MODEL_NAME,
                target_type:  <$dest as  $crate::charybdis::model::BaseModel>::DB_MODEL_NAME,
                edge_store_type: ::hoover3_types::db_schema::EdgeStoreImplementation::Stored,
            });
            impl $crate::models::collection::GraphEdge for $struct_name {
                type SourceType = $source;
                type DestType = $dest;
                fn edge_type() ->  ::hoover3_types::db_schema::GraphEdgeId {
                    ::hoover3_types::db_schema::GraphEdgeId(
                        ::hoover3_types::identifier::DatabaseIdentifier::new($edge_name)
                            .expect("invalid edge name: is not DatabaseIdentifier"),
                    )
                }
            }
            #[allow(non_snake_case)]
            mod [<__$struct_name>] {
                use $crate::models::collection::GraphEdgeQuery;
                use $crate::models::collection::GraphEdgeInsert;
                use $crate::models::collection::EdgeBatchOperation;
                use ::hoover3_types::identifier::CollectionId;
                use $crate::charybdis::model::BaseModel;
                use std::future::Future;
                use $crate::models::collection::ResultStream;
                use $crate::models::collection::edge_list_targets_pk;
                use $crate::models::collection::edge_list_source_pk;
                use $crate::models::collection::pull_full_models;
                use std::pin::Pin;
                use futures::FutureExt;
                impl GraphEdgeInsert for super::$struct_name {
                    fn edge_batch( collection_id: &CollectionId) -> EdgeBatchOperation<Self> {
                        EdgeBatchOperation::new(collection_id.clone())
                    }
                }

                impl GraphEdgeQuery for super::$struct_name {
                    fn list_target(
                        &self,
                        collection_id: &CollectionId,
                        source: &<Self::SourceType as BaseModel>::PrimaryKey,
                    ) -> Pin<Box<dyn Future<Output = anyhow::Result<ResultStream<Self::DestType>>>+Send>>
                    {
                        let collection_id = collection_id.clone();
                        let source = source.clone();
                        async move {

                            Ok(pull_full_models(
                                &collection_id,
                                edge_list_targets_pk::<Self>(
                                    &collection_id,
                                    &source
                                ).await?
                            ).await?)
                        }.boxed()
                    }

                    fn list_source(
                        &self,
                        collection_id: &CollectionId,
                        target: &<Self::DestType as BaseModel>::PrimaryKey,
                    ) -> Pin<Box<dyn Future<Output = anyhow::Result<ResultStream<Self::SourceType>>>+Send>>
                    {
                        let collection_id = collection_id.clone();
                        let target = target.clone();
                        async move {
                            Ok(pull_full_models(
                                &collection_id,
                                edge_list_source_pk::<Self>(
                                    &collection_id,
                                    &target
                                ).await?
                            ).await?)
                        }.boxed()
                    }

                }
            }
        }
    };
}
// pub use declare_stored_graph_edge;

#[macro_export]
macro_rules! declare_implicit_graph_edge {
    ($struct_name:ident, $edge_name:expr, $source:ty, $dest:ty) => {
        $crate::paste::paste! {

            /// Unit struct to identify an implicit graph edge `$id``.
            pub struct $struct_name;

            $crate::inventory::submit!($crate::models::collection::GraphEdgeTypeStatic {
                edge_type: $edge_name,
                source_type: <$source as  $crate::charybdis::model::BaseModel>::DB_MODEL_NAME,
                target_type:  <$dest as  $crate::charybdis::model::BaseModel>::DB_MODEL_NAME,
                edge_store_type: ::hoover3_types::db_schema::EdgeStoreImplementation::Implicit,
            });
            impl $crate::models::collection::GraphEdge for $struct_name {
                type SourceType = $source;
                type DestType = $dest;
                fn edge_type() ->  ::hoover3_types::db_schema::GraphEdgeId {
                    ::hoover3_types::db_schema::GraphEdgeId(
                        ::hoover3_types::identifier::DatabaseIdentifier::new($edge_name)
                            .expect("invalid edge name: is not DatabaseIdentifier"),
                    )
                }
            }
            impl $crate::models::collection::GraphEdgeImplicit for $struct_name {}
        }
    };
}

#[derive(Clone)]
pub struct GraphEdgeChainQuery<E1, E2>
where
    E1: GraphEdgeQuery,
    E2: GraphEdgeQuery,
    E1::DestType: TypeEquals<Other = E2::SourceType>,
{
    e1: Arc<dyn GraphEdgeQuery<SourceType = E1::SourceType, DestType = E1::DestType>>,
    e2: Arc<dyn GraphEdgeQuery<SourceType = E2::SourceType, DestType = E2::DestType>>,
}
pub fn chain_edges<E1, E2>(e1: E1, e2: E2) -> GraphEdgeChainQuery<E1, E2>
where
    E1: GraphEdgeQuery,
    E2: GraphEdgeQuery,
    E1::DestType: TypeEquals<Other = E2::SourceType>,
{
    GraphEdgeChainQuery {
        e1: Arc::new(e1),
        e2: Arc::new(e2),
    }
}
impl<E1, E2> GraphEdge for GraphEdgeChainQuery<E1, E2>
where
    E1: GraphEdgeQuery,
    E2: GraphEdgeQuery,
    E1::DestType: TypeEquals<Other = E2::SourceType>,
{
    type SourceType = E1::SourceType;
    type DestType = E2::DestType;
    fn edge_type() -> GraphEdgeId {
        GraphEdgeId(
            DatabaseIdentifier::new(format!("{}__{}", E1::edge_type(), E2::edge_type())).unwrap(),
        )
    }
}
impl<E1, E2> GraphEdgeQuery for GraphEdgeChainQuery<E1, E2>
where
    E1: GraphEdgeQuery,
    E2: GraphEdgeQuery,
    E1::DestType: TypeEquals<Other = E2::SourceType>,
{
    fn list_target(
        &self,
        collection_id: &CollectionId,
        source: &<Self::SourceType as BaseModel>::PrimaryKey,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<ResultStream<Self::DestType>>> + Send>> {
        let e1 = self.e1.clone();
        let e2 = self.e2.clone();
        let source = source.clone();
        let collection_id = collection_id.clone();
        async move {
            let stream = try_stream! {
                let mid_stream = e1.list_target(&collection_id, &source).await?;
                pin_mut!(mid_stream);
                while let Some(item) = mid_stream.next().await {
                    let item = item?;
                    let v = item.primary_key_values();
                    let v = &v as &dyn std::any::Any;
                    let v = v.downcast_ref::<<E2::SourceType as BaseModel>::PrimaryKey>().unwrap();
                    let dest_stream = e2.list_target(&collection_id, v).await?;
                    pin_mut!(dest_stream);
                    while let Some(dest_item) = dest_stream.next().await {
                        let dest_item = dest_item?;
                        yield dest_item;
                    }
                }
            };

            anyhow::Ok(stream.boxed())
        }
        .boxed()
    }

    fn list_source(
        &self,
        collection_id: &CollectionId,
        target: &<Self::DestType as BaseModel>::PrimaryKey,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<ResultStream<Self::SourceType>>> + Send>> {
        let e1 = self.e1.clone();
        let e2 = self.e2.clone();
        let target = target.clone();
        let collection_id = collection_id.clone();
        async move {
            let stream = try_stream! {
                let mid_stream = e2.list_source(&collection_id, &target).await?;
                pin_mut!(mid_stream);
                while let Some(item) = mid_stream.next().await {
                    let item = item?;
                    let v = item.primary_key_values();
                    let v = &v as &dyn std::any::Any;
                    let v = v.downcast_ref::<<E1::DestType as BaseModel>::PrimaryKey>().unwrap();
                    let dest_stream = e1.list_source(&collection_id, v).await?;
                    pin_mut!(dest_stream);
                    while let Some(dest_item) = dest_stream.next().await {
                        let dest_item = dest_item?;
                        yield dest_item;
                    }
                }
            };

            anyhow::Ok(stream.boxed())
        }
        .boxed()
    }
}

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

    #[charybdis_model(
        table_name = test_graph_parent_child,
        partition_keys = [id],
        clustering_keys = [second]
    )]
    pub struct TestChildObject {
        pub id: Text,
        pub second: Text,
    }

    #[charybdis_model(
        table_name = test_graph_parent_child,
        partition_keys = [id, second],
        clustering_keys = [third]
    )]
    pub struct TestGrandchildObject {
        pub id: Text,
        pub second: Text,
        pub third: Text,
    }

    declare_stored_graph_edge!(TestEdge, "graph_test_edge", TestModel, TestModel);
    declare_implicit_graph_edge!(
        TestImplicitEdge,
        "graph_test_implicit_edge",
        TestModel,
        TestChildObject
    );
    declare_implicit_graph_edge!(
        TestImplicitEdge2,
        "graph_test_implicit_edge2",
        TestChildObject,
        TestGrandchildObject
    );

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
        let _chain = chain_edges(TestImplicitEdge, TestImplicitEdge2);
        // don't actually await the future, we don't have a collection here - just drop it
        let _ = _chain.list_target(
            &CollectionId::new("dummy").unwrap(),
            &("dummy".to_string(),),
        );
        let _ = _chain.list_source(
            &CollectionId::new("dummy").unwrap(),
            &(
                "dummy".to_string(),
                "dummy".to_string(),
                "dummy".to_string(),
            ),
        );

        let _chain = chain_edges(TestEdge, _chain);
        let _ = _chain.list_target(
            &CollectionId::new("dummy").unwrap(),
            &("dummy".to_string(),),
        );
        let _ = _chain.list_source(
            &CollectionId::new("dummy").unwrap(),
            &(
                "dummy".to_string(),
                "dummy".to_string(),
                "dummy".to_string(),
            ),
        );
    }
}
