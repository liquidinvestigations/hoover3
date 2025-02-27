use futures::StreamExt;
use hoover3_types::identifier::CollectionId;
use scylla::batch::{Batch, BatchType};
use std::collections::HashMap;
use charybdis::batch::ModelBatch;
use std::collections::HashSet;
use crate::db_management::{DatabaseSpaceManager, ScyllaDatabaseHandle};
use super::_scylla_graph_models::*;

const PAGE_SIZE: i32 = 1000;

/// Adds multiple edges of the same type to the graph database.
///
/// This function:
/// 1. Inserts edges into the GraphEdgePage table
/// 2. Increments the appropriate counters in GraphEdgePagesCounter
///
/// # Arguments
///
/// * `collection_id` - The ID of the collection
/// * `edge_type` - The type of all edges being added
/// * `edges` - A vector of tuples containing (pk_source, pk_target)
/// * `direction_out` - Edge direction flag (true = OUT, false = IN)
///
/// # Returns
///
/// A Result containing the number of edges added or an error
pub async fn add_edges(
    collection_id: CollectionId,
    edge_type: String,
    edges: Vec<(String, String)>,
    direction_out: bool,
) -> Result<usize, anyhow::Error> {
    if edges.is_empty() {
        return Ok(0);
    }

    // Get session early to query existing counters
    let session = ScyllaDatabaseHandle::collection_session(&collection_id).await?;

    // Group edges by source for counter updates
    let mut edge_counters: HashMap<String, i64> = HashMap::new();

    // First, collect all unique sources
    let mut unique_sources: HashSet<String> = HashSet::new();
    for (source, _) in &edges {
        unique_sources.insert(source.clone());
    }
    tracing::info!("add_edges: unique_sources: {:?}", unique_sources.len());

    // Query existing counters from database - optimize with batch IN queries
    // Process in chunks of 100 (ScyllaDB's IN clause limit)
    for source_chunk in unique_sources.iter().collect::<Vec<_>>().chunks(100) {
        let mut counters = find_graph_edge_pages_counter!(
            "pk_source IN ? AND edge_type = ? AND direction_out = ?",
            (source_chunk, edge_type.clone(), direction_out)
        )
        .execute(&session)
        .await?;

        // Add found counters to our map
        while let Some(Ok(counter)) = counters.next().await {
            edge_counters.insert(counter.pk_source.clone(), counter.item_count.0);
        }

        // Initialize missing counters with 0
        for source in source_chunk {
            edge_counters.entry((*source).clone()).or_insert(0);
        }
    }

    // Track which pages need to be updated
    let mut page_assignments: HashMap<String, HashMap<i32, Vec<String>>> = HashMap::new();

    // First pass: count edges and assign to pages
    for (source, target) in &edges {
        // Get current count to determine page assignment
        let current_count = *edge_counters.get(source).unwrap();
        let page_id = current_count / PAGE_SIZE as i64;

        // Add target to the appropriate page
        page_assignments
            .entry(source.clone())
            .or_insert_with(HashMap::new)
            .entry(page_id as i32)
            .or_insert_with(Vec::new)
            .push(target.clone());

        // Increment counter for this source
        *edge_counters.entry(source.clone()).or_insert(0) += 1;
    }
    tracing::info!("add_edges: page_assignments: {:?}", page_assignments.len());
    // Build and execute queries using charybdis batch
    let mut edge_pages = Vec::new();

    // Add edge pages
    for (source, pages) in page_assignments {
        for (page_id, targets) in pages {
            for target in targets {
                edge_pages.push(GraphEdgePage {
                    pk_source: source.clone(),
                    edge_type: edge_type.clone(),
                    direction_out,
                    page_id,
                    pk_target: target,
                });
            }
        }
    }

    // Count how many edges are being added for each source
    let mut counter_increments: HashMap<String, i64> = HashMap::new();
    for (source, _) in &edges {
        *counter_increments.entry(source.clone()).or_insert(0) += 1;
    }

    let mut scylla_counter_batch = Batch::new(BatchType::Counter);
    let mut batch_values = vec![];
    let q = format!("
    UPDATE {}.graph_edge_pages_counter
      SET item_count = item_count + ?
      WHERE pk_source = ?
      AND edge_type = ?
      AND direction_out = ?;
      ", collection_id.database_name()?.to_string());
    tracing::info!("add_edges: q: {}", q);
    let q = session.get_session().prepare(q).await?;

    for (source, increment) in counter_increments {
        scylla_counter_batch.append_statement(q.clone());
        batch_values.push((scylla::frame::value::Counter(increment), source, edge_type.clone(), direction_out));
    }
    tracing::info!("add_edges: batch_values: {:?}", batch_values.len());
    session.batch(&scylla_counter_batch, &batch_values).await?;
    // session.execute_unpaged(query_batch, &[]).await?;

    // Insert edge pages in chunks
    if !edge_pages.is_empty() {
        GraphEdgePage::batch()
            .chunked_insert(&session, &edge_pages, 1024)
            .await?;
    }

    Ok(edges.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::TryStreamExt;
    use hoover3_tracing::init_tracing;
    use hoover3_types::identifier::CollectionId;
    use crate::client_query::collections::{create_new_collection, drop_collection};

    #[tokio::test]
    async fn test_add_edges_basic() -> Result<(), anyhow::Error> {
        init_tracing();

        // Create a test collection
        let collection_id = CollectionId::new("test_add_edges_basic")?;

        // Clean up previous tests
        drop_collection(collection_id.clone()).await?;

        create_new_collection(collection_id.clone()).await?;

        // Define simple test data - just one edge
        let edge_type = "LINKS_TO".to_string();
        let direction_out = true;
        let edges = vec![("doc1".to_string(), "doc2".to_string())];

        // Add the edge
        let result = add_edges(
            collection_id.clone(),
            edge_type.clone(),
            edges.clone(),
            direction_out,
        ).await?;

        // Verify correct number of edges were added
        assert_eq!(result, 1);

        // Get session to query the database
        let session = ScyllaDatabaseHandle::collection_session(&collection_id).await?;

        // Check counter was initialized properly
        let counter = find_graph_edge_pages_counter!(
            "pk_source = ? AND edge_type = ? AND direction_out = ?",
            ("doc1", edge_type.clone(), direction_out)
        )
        .execute(&session)
        .await?
        .try_next()
        .await?
        .expect("Counter should exist");

        // Verify counter properties
        assert_eq!(counter.pk_source, "doc1");
        assert_eq!(counter.edge_type, edge_type);
        assert_eq!(counter.direction_out, direction_out);
        assert_eq!(counter.item_count.0, 1);

        // Clean up
        drop_collection(collection_id).await?;

        Ok(())
    }
}
