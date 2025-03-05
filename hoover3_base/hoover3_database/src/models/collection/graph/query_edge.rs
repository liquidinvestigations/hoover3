use super::GraphEdge;
use crate::models::collection::GraphNodePkMap;
use crate::{
    db_management::{DatabaseSpaceManager, ScyllaDatabaseHandle},
    models::collection::{
        find_graph_node_pk_map, graph_models::GraphEdgePageContent, row_pk_hash, GraphEdgePageList,
    },
};
use async_stream::try_stream;
use charybdis::model::BaseModel;
use charybdis::operations::Find;
use futures::{pin_mut, stream::Stream, StreamExt};
use hoover3_types::{db_schema::GraphEdgeType, identifier::CollectionId};
use serde::{Deserialize, Serialize};
use std::pin::Pin;

/// Go over edge E in the forward direction
/// from a source node, and return a stream of all the target nodes.
pub async fn graph_edge_targets_for_source<E: GraphEdge>(
    collection_id: &CollectionId,
    source: &<E::SourceType as BaseModel>::PrimaryKey,
) -> anyhow::Result<ResultStream<<E::DestType as BaseModel>::PrimaryKey>>
where
    <E::SourceType as BaseModel>::PrimaryKey:
        Send + Sync + 'static + Serialize + for<'a> Deserialize<'a>,
    <E::DestType as BaseModel>::PrimaryKey:
        Send + Sync + 'static + Serialize + for<'a> Deserialize<'a>,
{
    let source_pk_hash = row_pk_hash::<E::SourceType>(&source);
    let stream =
        list_edges_of_type(collection_id.clone(), E::edge_type(), true, source_pk_hash).await?;
    let stream = stream.map(|value_json| {
        let value: <E::DestType as BaseModel>::PrimaryKey = serde_json::from_str(&value_json?)?;
        Ok(value)
    });
    Ok(Box::pin(stream))
}

/// Go over edge E in the reverse direction
/// from a target node, and return a stream of all the source nodes.
pub async fn graph_edge_sources_for_target<E: GraphEdge>(
    collection_id: &CollectionId,
    target: &<E::DestType as BaseModel>::PrimaryKey,
) -> anyhow::Result<ResultStream<<E::SourceType as BaseModel>::PrimaryKey>>
where
    <E::SourceType as BaseModel>::PrimaryKey:
        Send + Sync + 'static + Serialize + for<'a> Deserialize<'a>,
    <E::DestType as BaseModel>::PrimaryKey:
        Send + Sync + 'static + Serialize + for<'a> Deserialize<'a>,
{
    let target_pk_hash = row_pk_hash::<E::DestType>(&target);
    let stream =
        list_edges_of_type(collection_id.clone(), E::edge_type(), false, target_pk_hash).await?;
    let stream = stream.map(|value_json| {
        let value: <E::SourceType as BaseModel>::PrimaryKey = serde_json::from_str(&value_json?)?;
        Ok(value)
    });
    Ok(Box::pin(stream))
}

async fn list_edges_of_type(
    collection_id: CollectionId,
    edge_type: GraphEdgeType,
    direction_out: bool,
    source_pk_hash: String,
) -> anyhow::Result<ResultStream<String>> {
    let session = ScyllaDatabaseHandle::collection_session(&collection_id).await?;

    let stream = try_stream! {
        let pk_hash_stream =
            list_edge_targets(
                collection_id,
                edge_type.clone(),
                direction_out,
                source_pk_hash
            ).await?;
        let pk_hash_stream = chunk_stream(pk_hash_stream, 100);
        pin_mut!(pk_hash_stream);

        while let Some(targets_chunk) = pk_hash_stream.next().await {
            let targets_chunk = targets_chunk?;
            let pk_maps = find_graph_node_pk_map!(
                "pk in ?",
                (targets_chunk,)
            ).execute(&session).await?;
            pin_mut!(pk_maps);
            while let Some(pk_map) = pk_maps.next().await {
                let value_json = pk_map?.value;
                yield value_json;
            }
        }
    };
    Ok(Box::pin(stream))
}

/// A fallible stream of items.
pub type ResultStream<T> = Pin<Box<dyn Stream<Item = anyhow::Result<T>> + Send>>;

fn chunk_stream<T: Send + 'static>(
    stream: ResultStream<T>,
    chunk_size: usize,
) -> ResultStream<Vec<T>> {
    let stream = try_stream! {
        let mut chunk = Vec::new();
        pin_mut!(stream);
        while let Some(item) = stream.next().await {
            chunk.push(item?);
            if chunk.len() >= chunk_size {
                yield std::mem::replace(&mut chunk, vec![]);
            }
        }
        if !chunk.is_empty() {
            yield chunk;
        }
    };
    Box::pin(stream)
}

/// Returns a stream of target node primary key hashes for a given
/// source, edge type, and direction.
///
/// # Arguments
/// * `collection_id` - The collection ID
/// * `edge_type` - The type of edge to query
/// * `direction_out` - The direction of the edge (true = outgoing, false = incoming)
/// * `source_pk_hash` - The source node's primary key hash as a string
///
/// # Returns
/// A stream of chunks containing target node primary key hashes as strings.
/// These can be looked up in the database model [GraphNodePkMap] to get the primary fields
/// of the target node.
async fn list_edge_targets(
    collection_id: CollectionId,
    edge_type: GraphEdgeType,
    direction_out: bool,
    source_pk_hash: String,
) -> anyhow::Result<ResultStream<String>> {
    let session = ScyllaDatabaseHandle::collection_session(&collection_id).await?;
    let edge_type_str = edge_type.to_string();

    // Create a stream using async_stream's try_stream macro
    let stream = try_stream! {
        // Get all pages for this source, edge type, and direction
        let page_futures = GraphEdgePageList::find_by_partition_key_value(
            (source_pk_hash.clone(), edge_type_str.clone(), direction_out)
        ).execute(&session).await?;
        pin_mut!(page_futures);

        // Process each page
        while let Some(page_result) = page_futures.next().await {
            let page = page_result?;

            // Query the GraphEdgePageContent table to get all targets for this page
            let content_futures = GraphEdgePageContent::find_by_partition_key_value(
                (source_pk_hash.clone(), edge_type_str.clone(), direction_out, page.page_id)
            ).execute(&session).await?;

            pin_mut!(content_futures);

            // Collect all target PKs from this page
            while let Some(edge) = content_futures.next().await {
                yield (edge?.pk_target);
            }
        }
    };

    Ok(Box::pin(stream))
}
