use super::_declare_edge::GraphEdge;
use super::_scylla_graph_models::*;
use super::row_pk_hash;
use crate::db_management::{DatabaseSpaceManager, ScyllaDatabaseHandle};
use charybdis::{batch::ModelBatch, model::BaseModel};
use futures::pin_mut;
use futures::FutureExt;
use futures::{stream::FuturesUnordered, StreamExt};
use hoover3_types::db_schema::GraphEdgeType;
use hoover3_types::identifier::CollectionId;

use scylla::batch::{Batch, BatchType};
use std::collections::HashMap;
use std::collections::HashSet;

/// The number of edges to insert into a single page in the GraphEdgePage table.
/// The actual scylla limit is 100k, but our counters aren't atomic,
/// so we want a large margin
const CQL_TARGET_PARTITION_SIZE: i32 = 10000;

/// The Cql limit for SELECT ... WHERE field IN ? queries.
const CQL_SELECT_BATCH_SIZE: usize = 100;

/// The number of batches to run in parallel for a single operation.
const CQL_PARALLEL_BATCHES: usize = 8;

/// Add nodes to graph. These are just mappings from a "row_pk" to the primary keys of the table
pub async fn graph_add_nodes<T>(
    collection_id: &CollectionId,
    nodes: &[T],
) -> Result<usize, anyhow::Error>
where
    T: BaseModel + Send + Sync + 'static,
    <T as BaseModel>::PrimaryKey: serde::Serialize + 'static,
    <T as BaseModel>::PrimaryKey: for<'a> serde::Deserialize<'a>,
{
    let nodes = nodes
        .into_iter()
        .map(|t| {
            let row_pk = row_pk_hash::<T>(&t.primary_key_values());
            let row_val = serde_json::to_value(t.primary_key_values())?;

            anyhow::Ok(GraphNodePkMap {
                pk: row_pk,
                value: row_val.to_string(),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let session = ScyllaDatabaseHandle::collection_session(&collection_id).await?;
    GraphNodePkMap::batch()
        .chunked_insert(&session, &nodes, 1024)
        .await?;
    Ok(nodes.len())
}

/// A batch for inserting edges into the graph.
pub struct EdgeBatchOperation<E: GraphEdge> {
    collection_id: CollectionId,
    _ph: std::marker::PhantomData<E>,
    edges: Vec<(String, String)>,
}

impl<E: GraphEdge> EdgeBatchOperation<E> {
    /// Create a new batch operation for inserting edges into the graph.
    pub(crate) fn new(collection_id: CollectionId) -> Self {
        Self {
            collection_id,
            edges: Vec::new(),
            _ph: std::marker::PhantomData,
        }
    }

    /// Execute the batch operation.
    pub async fn execute(&self) -> anyhow::Result<usize> {
        graph_add_edges(
            self.collection_id.clone(),
            E::edge_type(),
            self.edges.clone(),
        )
        .await
    }
}
impl<E: GraphEdge> EdgeBatchOperation<E>
where
    E::SourceType: BaseModel + Send + Sync,
    E::DestType: BaseModel + Send + Sync,
    <E::SourceType as BaseModel>::PrimaryKey:
        serde::Serialize + for<'a> serde::Deserialize<'a> + 'static,
    <E::DestType as BaseModel>::PrimaryKey:
        serde::Serialize + for<'a> serde::Deserialize<'a> + 'static,
{
    /// Add edge to batch, using references to models.
    pub fn add_edge(&mut self, source: &E::SourceType, dest: &E::DestType) {
        let s = row_pk_hash::<E::SourceType>(&source.primary_key_values());
        let d = row_pk_hash::<E::DestType>(&dest.primary_key_values());
        self.edges.push((s, d));
    }

    /// Add edge to batch, using references to primary keys.
    pub fn add_edge_from_pk(
        &mut self,
        source: &<E::SourceType as BaseModel>::PrimaryKey,
        dest: &<E::DestType as BaseModel>::PrimaryKey,
    ) {
        let s = row_pk_hash::<E::SourceType>(source);
        let d = row_pk_hash::<E::DestType>(dest);
        self.edges.push((s, d));
    }
}

/// Add many edges to graph for a specific edge type.
/// Internally, they are batched and inserted in parallel.
/// Returns the number of edges added or an error.
async fn graph_add_edges(
    collection_id: CollectionId,
    edge_type: GraphEdgeType,
    edges: Vec<(String, String)>,
) -> Result<usize, anyhow::Error> {
    let edge_type = edge_type.0.to_string();
    let mut futures = FuturesUnordered::new();
    let mut count = 0;
    // let edge_type = edge_type.to_owned().name.to_string();
    for edge_chunk in edges.chunks(CQL_SELECT_BATCH_SIZE) {
        futures.push(add_edges_single_batch(
            collection_id.clone(),
            edge_type.clone(),
            edge_chunk.to_vec(),
            true,
        ));
        let edge_chunk_rev = edge_chunk
            .iter()
            .map(|(a, b)| (b.clone(), a.clone()))
            .collect();
        futures.push(add_edges_single_batch(
            collection_id.clone(),
            edge_type.clone(),
            edge_chunk_rev,
            false,
        ));
        while futures.len() > CQL_PARALLEL_BATCHES {
            let result = futures.next().await.unwrap();
            count += result?;
        }
    }
    while let Some(result) = futures.next().await {
        count += result?;
    }
    Ok(count)
}

/// Adds multiple edges of the same type to the graph database.
/// If they aready exist, they are not added again.
/// Returns the number of edges added or an error.
async fn add_edges_single_batch(
    collection_id: CollectionId,
    edge_type: String,
    edges: Vec<(String, String)>,
    direction_out: bool,
) -> Result<usize, anyhow::Error> {
    if edges.is_empty() {
        return Ok(0);
    }

    let edges = skip_existing_edges(&collection_id, &edge_type, &edges, direction_out).await?;

    if edges.is_empty() {
        return Ok(0);
    }

    let page_assignments =
        add_edges_get_page_assignments(&collection_id, &edge_type, &edges, direction_out).await?;

    if page_assignments.is_empty() {
        return Ok(0);
    }

    // Build and execute queries using charybdis batch
    let mut edge_pages_rows = Vec::new();
    let mut edge_page_assign_rows = Vec::new();

    // Add edge pages
    for (source, pages) in page_assignments {
        for (page_id, targets) in pages {
            for target in targets {
                edge_pages_rows.push(GraphEdgePage {
                    pk_source: source.clone(),
                    edge_type: edge_type.clone(),
                    direction_out,
                    page_id,
                    pk_target: target.clone(),
                });
                edge_page_assign_rows.push(GraphEdgePageAssignment {
                    edge_pks: (source.clone(), target.clone()),
                    edge_type: edge_type.clone(),
                    direction_out,
                    page_id,
                });
            }
        }
    }

    let session = ScyllaDatabaseHandle::collection_session(&collection_id).await?;
    GraphEdgePage::batch()
        .chunked_insert(&session, &edge_pages_rows, 1024)
        .await?;

    GraphEdgePageAssignment::batch()
        .chunked_insert(&session, &edge_page_assign_rows, 1024)
        .await?;

    let edge_count = edges.len();
    add_edges_batch_increment_counters(&collection_id, &edge_type, &edges, direction_out).await?;

    Ok(edge_count)
}

async fn add_edges_get_page_assignments(
    collection_id: &CollectionId,
    edge_type: &str,
    edges: &[(String, String)],
    direction_out: bool,
) -> anyhow::Result<HashMap<String, HashMap<i32, Vec<String>>>> {
    // Group edges by source for counter updates
    let mut edge_counters: HashMap<String, i64> = HashMap::new();

    // First, collect all unique sources
    let mut unique_sources: HashSet<String> = HashSet::new();
    for (source, _) in edges.iter() {
        unique_sources.insert(source.clone());
    }
    tracing::info!("add_edges: unique_sources: {:?}", unique_sources.len());
    // Track which pages need to be updated
    let mut page_assignments: HashMap<String, HashMap<i32, Vec<String>>> = HashMap::new();

    // Query existing counters from database - optimize with batch IN queries
    // Process in chunks of 100 (ScyllaDB's IN clause limit)
    let session = ScyllaDatabaseHandle::collection_session(&collection_id).await?;
    for source_chunk in unique_sources
        .iter()
        .collect::<Vec<_>>()
        .chunks(CQL_SELECT_BATCH_SIZE)
    {
        let counters = find_graph_edge_pages_counter!(
            "pk_source IN ? AND edge_type = ? AND direction_out = ?",
            (source_chunk, edge_type.to_string(), direction_out)
        )
        .execute(&session)
        .boxed()
        .await?;
        pin_mut!(counters);

        // Add found counters to our map
        while let Some(Ok(counter)) = counters.next().await {
            edge_counters.insert(counter.pk_source.clone(), counter.item_count.0);
        }

        // Initialize missing counters with 0
        for source in source_chunk {
            edge_counters.entry((*source).clone()).or_insert(0);
        }
    }

    // First pass: count edges and assign to pages
    for (source, target) in edges.iter() {
        // Get current count to determine page assignment
        let current_count = *edge_counters.get(source).unwrap();
        let page_id = current_count / CQL_TARGET_PARTITION_SIZE as i64;

        // Add target to the appropriate page
        page_assignments
            .entry(source.clone())
            .or_insert_with(HashMap::new)
            .entry(page_id as i32)
            .or_insert_with(Vec::new)
            .push(target.clone());

        // Increment counter for this source, to change the page assignment
        *edge_counters.entry(source.clone()).or_insert(0) += 1;
    }
    tracing::info!("add_edges: page_assignments: {:?}", page_assignments.len());
    Ok(page_assignments)
}

/// Increments the counters for the edges being added.
async fn add_edges_batch_increment_counters(
    collection_id: &CollectionId,
    edge_type: &str,
    edges: &[(String, String)],
    direction_out: bool,
) -> Result<(), anyhow::Error> {
    let session = ScyllaDatabaseHandle::collection_session(collection_id).await?;

    // Count how many edges are being added for each source
    let mut counter_increments: HashMap<String, i64> = HashMap::new();
    for (source, _) in edges.iter() {
        *counter_increments.entry(source.clone()).or_insert(0) += 1;
    }

    let mut scylla_counter_batch = Batch::new(BatchType::Counter);
    let mut batch_values = vec![];
    let q = format!(
        "
    UPDATE {}.graph_edge_pages_counter
      SET item_count = item_count + ?
      WHERE pk_source = ?
      AND edge_type = ?
      AND direction_out = ?;
      ",
        collection_id.database_name()?.to_string()
    );
    tracing::info!("add_edges: q: {}", q);
    let q = session.get_session().prepare(q).await?;

    for (source, increment) in counter_increments {
        scylla_counter_batch.append_statement(q.clone());
        batch_values.push((
            scylla::frame::value::Counter(increment),
            source,
            edge_type.to_string(),
            direction_out,
        ));
    }
    tracing::info!("add_edges: batch_values: {:?}", batch_values.len());
    session.batch(&scylla_counter_batch, &batch_values).await?;
    Ok(())
}

/// Filters out edges that already exist in the database.
/// Returns vector of edges that don't already exist in the database
async fn skip_existing_edges(
    collection_id: &CollectionId,
    edge_type: &str,
    edges: &[(String, String)],
    direction_out: bool,
) -> Result<Vec<(String, String)>, anyhow::Error> {
    if edges.is_empty() {
        return Ok(Vec::new());
    }

    let session = ScyllaDatabaseHandle::collection_session(&collection_id).await?;

    let mut already_exists = HashSet::new();

    // Process targets in chunks to avoid exceeding IN clause limits
    for target_chunk in edges.chunks(CQL_SELECT_BATCH_SIZE) {
        // Query existing edges for this source and target chunk
        let mut existing_edges = find_graph_edge_page_assignment!(
            "edge_pks IN ? AND edge_type = ? AND direction_out = ?",
            (target_chunk.to_vec(), edge_type.to_string(), direction_out)
        )
        .execute(&session)
        .await?;

        while let Some(Ok(edge)) = existing_edges.next().await {
            already_exists.insert(edge.edge_pks);
        }
    }

    tracing::info!(
        "skip_existing_edges: filtered {}/{} edges",
        already_exists.len(),
        edges.len(),
    );

    Ok(edges
        .into_iter()
        .filter(|(source, target)| !already_exists.contains(&(source.clone(), target.clone())))
        .cloned()
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client_query::collections::{create_new_collection, drop_collection},
        migrate::migrate_common,
    };
    use futures::{FutureExt, TryStreamExt};
    use hoover3_tracing::init_tracing;
    use hoover3_types::identifier::CollectionId;

    async fn create_test_collection(name: &str) -> Result<CollectionId, anyhow::Error> {
        init_tracing();
        migrate_common().await?;
        let collection_id = CollectionId::new(name)?;
        drop_collection(collection_id.clone()).await?;
        create_new_collection(collection_id.clone()).await?;
        Ok(collection_id)
    }

    #[tokio::test]
    async fn test_add_edges_basic() -> Result<(), anyhow::Error> {
        // Create a test collection
        let collection_id =
            tokio::spawn(async move { create_test_collection("test_add_edges_basic").await })
                .await??;

        // Define simple test data - just one edge
        let edge_type = "LINKS_TO".to_string();
        let direction_out = true;
        let edges = vec![("doc1".to_string(), "doc2".to_string())];

        // Add the edge
        let collection_id_ = collection_id.clone();
        let edge_type_ = edge_type.clone();
        let edges_ = edges.clone();
        let direction_out_ = direction_out;
        let result = tokio::spawn(
            async move {
                add_edges_single_batch(collection_id_, edge_type_, edges_, direction_out_).await
            }
            .boxed(),
        )
        .await??;
        // let result = add_edges_single_batch(
        //     collection_id.clone(),
        //     edge_type.clone(),
        //     edges.clone(),
        //     direction_out,
        // )
        // .await?;

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

        // ========================

        // do it all a second time - should be no-ops
        let result = add_edges_single_batch(
            collection_id.clone(),
            edge_type.clone(),
            edges.clone(),
            direction_out,
        )
        .await?;
        assert_eq!(result, 0);

        // check the counter is still 1
        let counter = find_graph_edge_pages_counter!(
            "pk_source = ? AND edge_type = ? AND direction_out = ?",
            ("doc1", edge_type.clone(), direction_out)
        )
        .execute(&session)
        .await?
        .try_next()
        .await?
        .expect("Counter should exist");
        assert_eq!(counter.item_count.0, 1);

        // Clean up
        drop_collection(collection_id).await?;

        Ok(())
    }
}
