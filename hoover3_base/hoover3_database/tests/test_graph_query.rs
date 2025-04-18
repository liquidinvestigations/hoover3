//! Integration tests for graph queries - test the public API for inserting and querying edges.

#![allow(missing_docs)]

use charybdis::model::BaseModel;
use charybdis::operations::InsertWithCallbacks;
use futures::StreamExt;
use hoover3_database::{
    client_query::collections::{create_new_collection, drop_collection},
    db_management::{DatabaseSpaceManager, ScyllaDatabaseHandle},
    declare_implicit_graph_edge, declare_stored_graph_edge,
    migrate::migrate_common,
    models::collection::*,
};
use hoover3_macro::model;
use hoover3_tracing::init_tracing;
use hoover3_types::identifier::CollectionId;

/// Docs
#[model]
pub struct TestModelA {
    /// Docs
    #[model(primary(partition))]
    pub id_a: String,
}

/// Docs
#[model]
pub struct TestModelB {
    /// Docs
    #[model(primary(partition))]
    pub id_b: String,
}

/// Docs
#[model]
pub struct TestModelChildB {
    /// Docs
    #[model(primary(partition))]
    pub id_b: String,
    /// Docs
    #[model(primary(clustering))]
    pub id_cluster: String,
}

/// Docs
#[model]
pub struct TestModelGrandchildB {
    /// Docs
    #[model(primary(partition))]
    pub id_b: String,

    /// Docs
    #[model(primary(partition))]
    pub id_cluster: String,

    /// Docs
    #[model(primary(clustering))]
    pub id_cluster2: String,

    /// Docs
    pub normal: String,
}
declare_stored_graph_edge!(TestModelEdge, "test_model_edge", TestModelA, TestModelB);
declare_stored_graph_edge!(
    TestModelEdge2,
    "test_model_edge2",
    TestModelChildB,
    TestModelB
);
declare_implicit_graph_edge!(
    TestImplicitEdge,
    "test_implicit_edge",
    TestModelB,
    TestModelChildB
);
declare_implicit_graph_edge!(
    TestImplicitEdge2,
    "test_implicit_edge2",
    TestModelChildB,
    TestModelGrandchildB
);

async fn create_test_collection(name: &str) -> Result<CollectionId, anyhow::Error> {
    init_tracing();
    migrate_common().await?;
    let collection_id = CollectionId::new(name)?;
    drop_collection(collection_id.clone()).await?;
    create_new_collection(collection_id.clone()).await?;
    Ok(collection_id)
}

#[tokio::test]
async fn test_graph_query() -> Result<(), anyhow::Error> {
    let c = create_test_collection("test_graph_query").await?;

    let mut test_model_a = TestModelA {
        id_a: "test_a".to_string(),
    };
    let mut test_model_b = TestModelB {
        id_b: "test_b".to_string(),
    };
    let mut test_model_child_b = TestModelChildB {
        id_b: "test_b".to_string(),
        id_cluster: "test_cluster".to_string(),
    };
    let mut test_model_grandchild_b = TestModelGrandchildB {
        id_b: "test_b".to_string(),
        id_cluster: "test_cluster".to_string(),
        id_cluster2: "test_cluster2".to_string(),
        normal: "test_normal".to_string(),
    };

    let cb = DatabaseExtraCallbacks::new(&c).await?;
    let session = ScyllaDatabaseHandle::collection_session(&c).await?;

    TestModelA::insert_cb(&mut test_model_a, &cb)
        .execute(&session)
        .await?;
    TestModelB::insert_cb(&mut test_model_b, &cb)
        .execute(&session)
        .await?;
    TestModelChildB::insert_cb(&mut test_model_child_b, &cb)
        .execute(&session)
        .await?;
    TestModelGrandchildB::insert_cb(&mut test_model_grandchild_b, &cb)
        .execute(&session)
        .await?;
    // ================================
    // Stored Edges
    // ================================
    let mut edges = TestModelEdge::edge_batch(&c);
    edges.add_edge(&test_model_a, &test_model_b);
    edges.execute().await?;

    let targets = TestModelEdge
        .list_target(&c, &test_model_a.primary_key_values())
        .await?;
    let targets = targets.collect::<Vec<_>>().await;
    assert_eq!(targets.len(), 1);
    let target0 = targets[0].as_ref().unwrap();
    assert_eq!(target0, &test_model_b);

    let sources = TestModelEdge
        .list_source(&c, &test_model_b.primary_key_values())
        .await?;
    let sources = sources.collect::<Vec<_>>().await;
    assert_eq!(sources.len(), 1);
    let s0 = sources[0].as_ref().unwrap();
    assert_eq!(s0, &test_model_a);
    // ================================
    // Implicit Edges
    // ================================
    let targets = TestImplicitEdge
        .list_target(&c, &test_model_b.primary_key_values())
        .await?;
    let targets = targets.collect::<Vec<_>>().await;
    assert_eq!(targets.len(), 1);
    let e0 = targets[0].as_ref().unwrap();
    assert_eq!(e0, &test_model_child_b);

    let sources = TestImplicitEdge
        .list_source(&c, &test_model_child_b.primary_key_values())
        .await?;
    let sources = sources.collect::<Vec<_>>().await;
    assert_eq!(sources.len(), 1);
    let s0 = sources[0].as_ref().unwrap();
    assert_eq!(s0, &test_model_b);

    let edges = TestImplicitEdge2
        .list_target(&c, &test_model_child_b.primary_key_values())
        .await?;
    let edges = edges.collect::<Vec<_>>().await;
    assert_eq!(edges.len(), 1);
    let e0 = edges[0].as_ref().unwrap();
    assert_eq!(e0, &test_model_grandchild_b);

    let sources = TestImplicitEdge2
        .list_source(&c, &test_model_grandchild_b.primary_key_values())
        .await?;
    let sources = sources.collect::<Vec<_>>().await;
    assert_eq!(sources.len(), 1);
    let s0 = sources[0].as_ref().unwrap();
    assert_eq!(s0, &test_model_child_b);

    drop_collection(c).await?;

    Ok(())
}
