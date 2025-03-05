//! Integration tests for graph queries - test the public API for inserting and querying edges.

#![allow(missing_docs)]

use charybdis::model::BaseModel;
use charybdis::operations::InsertWithCallbacks;
use futures::StreamExt;
use hoover3_database::{
    client_query::collections::{create_new_collection, drop_collection},
    db_management::{DatabaseSpaceManager, ScyllaDatabaseHandle},
    declare_graph_edge,
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

declare_graph_edge!(TestModelEdge, "test_model_edge", TestModelA, TestModelB);

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

    let cb = DatabaseExtraCallbacks::new(&c).await?;
    let session = ScyllaDatabaseHandle::collection_session(&c).await?;

    TestModelA::insert_cb(&mut test_model_a, &cb)
        .execute(&session)
        .await?;
    TestModelB::insert_cb(&mut test_model_b, &cb)
        .execute(&session)
        .await?;

    let mut edges = TestModelEdge::edge_batch(&c);
    edges.add_edge(&test_model_a, &test_model_b);
    edges.execute().await?;

    let targets =
        TestModelEdge::graph_edge_targets_for_source(&c, &test_model_a.primary_key_values())
            .await?;
    let targets = targets.collect::<Vec<_>>().await;
    assert_eq!(targets.len(), 1);
    let t0 = targets[0].as_ref().unwrap();
    assert_eq!(t0, &test_model_b.primary_key_values());

    Ok(())
}
