//! Plan for computing file hashes for all files in all collections.
//! Splits the work into chunks of similar file size.

use std::collections::BTreeMap;

use crate::models::{
    FsDatasourceToDirectory, FsDirectoryToFile, FsFileHashPlanDbRow, FsFileHashPlanPageDbRow,
};
use async_stream::try_stream;
use charybdis::operations::InsertWithCallbacks;
use futures::{pin_mut, StreamExt, TryStreamExt};
use hoover3_database::db_management::{DatabaseSpaceManager, ScyllaDatabaseHandle};
use hoover3_database::models::collection::{
    chain_edges, DatabaseExtraCallbacks, GraphEdgeQuery, ResultStream,
};
use hoover3_macro::activity;
use hoover3_tracing::tracing::info;
use hoover3_types::identifier::{CollectionId, DatabaseIdentifier};
use serde::{Deserialize, Serialize};

use super::FilesystemScannerQueue;

/// Compute the plan for hashing all files in a datasource. Write plan chunks to database.
/// Returns a list of plan chunk ids.
#[activity(FilesystemScannerQueue)]
pub async fn compute_file_hash_plan(
    (collection_id, datasource_id): (CollectionId, DatabaseIdentifier),
) -> anyhow::Result<Vec<i32>> {
    let mut stream = stream_file_hashing_plan(collection_id.clone(), datasource_id.clone()).await?;
    let mut plan_chunk_id = 0;
    let mut plan_chunk_ids = Vec::new();
    let db_extra = DatabaseExtraCallbacks::new(&collection_id).await?;
    let db_session = ScyllaDatabaseHandle::collection_session(&collection_id).await?;

    while let Some(chunk) = stream.next().await {
        plan_chunk_id += 1;
        let chunk = chunk?;
        let plan_data = serde_json::to_string(&chunk)?;
        let mut plan_page = FsFileHashPlanPageDbRow {
            datasource_id: datasource_id.to_string(),
            plan_chunk_id,
        };
        let mut plan = FsFileHashPlanDbRow {
            datasource_id: datasource_id.to_string(),
            plan_chunk_id,
            plan_data,
        };
        FsFileHashPlanPageDbRow::insert_cb(&mut plan_page, &db_extra)
            .execute(&db_session)
            .await?;
        FsFileHashPlanDbRow::insert_cb(&mut plan, &db_extra)
            .execute(&db_session)
            .await?;
        plan_chunk_ids.push(plan_chunk_id);
        info!(
            "PLAN CHUNK {}/{} CHUNK ID = {}",
            collection_id, datasource_id, plan_chunk_id
        );
    }

    Ok(plan_chunk_ids)
}

/// Chunk of work for computing file hashes.
#[derive(Debug, Serialize, Deserialize)]
pub struct FileHashPlanChunk {
    /// Sum of the sizes of all files in the chunk
    pub chunk_size: i64,
    /// Directories and file names in the chunk
    pub dirs: BTreeMap<String, Vec<(String, i64)>>,
}

/// Stream chunks of work for computing file hashes.
async fn stream_file_hashing_plan(
    collection_id: CollectionId,
    datasource_id: DatabaseIdentifier,
) -> anyhow::Result<ResultStream<FileHashPlanChunk>> {
    let min_read_size = 2_i64.pow(13); // 8 KB
    let max_chunk_size = 2_i64.pow(26); // 64 MB

    let edge_chain = chain_edges(FsDatasourceToDirectory, FsDirectoryToFile);
    let stream = edge_chain
        .list_target(&collection_id, &(datasource_id.to_string(),))
        .await?;
    let stream = chunk_by_size(stream, min_read_size, max_chunk_size, |file| {
        file.size_bytes
    })?;
    let stream = stream.map_ok(|chunk| {
        let mut dirs = BTreeMap::new();
        let mut chunk_size = 0;
        for item in chunk {
            let dir = item.parent_dir_path.clone();
            let file = item.file_name.clone();
            let size = item.size_bytes;
            dirs.entry(dir).or_insert(Vec::new()).push((file, size));
            chunk_size += size;
        }
        FileHashPlanChunk { dirs, chunk_size }
    });
    Ok(stream.boxed())
}

/// Chunk a stream of items by arbitrary item size.
/// Returns a stream of chunks, each containing a vector of items.
pub(crate) fn chunk_by_size<T: Send + Sync + 'static>(
    stream: ResultStream<T>,
    min_read_size: i64,
    max_chunk_size: i64,
    size_fn: impl Fn(&T) -> i64 + Send + Sync + 'static,
) -> Result<ResultStream<Vec<T>>, anyhow::Error> {
    Ok(try_stream! {
        pin_mut!(stream);

        let mut chunk_size = 0;
        let mut chunk = Vec::new();
        for await item in stream {
            let item = item?;
            let item_size = size_fn(&item).max(min_read_size);
            // if item is too big, yield it as a separate chunk, without editing current chunk
            if item_size > max_chunk_size/2 {
                yield vec![item];
                continue;
            }
            // add file to chunk
            chunk.push(item);
            chunk_size += item_size;
            if chunk_size > max_chunk_size {
                yield chunk;
                chunk_size = 0;
                chunk = Vec::new();
            }
        }
        if !chunk.is_empty() {
            yield chunk;
        }
    }
    .boxed())
}
