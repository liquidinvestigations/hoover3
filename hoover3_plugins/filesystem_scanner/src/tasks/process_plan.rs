//! Plan for processing blobs that have been hashed.
//! Splits the work into chunks of similar file size.

use crate::models::{
    BlobProcessingPlan, BlobProcessingPlanPage, FsBlobHashesDbRow, PartialUpdateFsBlobHashesDbRow,
};
use anyhow::Context;
use async_stream::try_stream;
use charybdis::batch::ModelBatch;
use charybdis::operations::{Find, InsertWithCallbacks};
use futures::{pin_mut, StreamExt, TryStreamExt};
use hoover3_database::db_management::{DatabaseSpaceManager, ScyllaDatabaseHandle};
use hoover3_database::models::collection::{DatabaseExtraCallbacks, ResultStream};
use hoover3_macro::{activity, workflow};
use hoover3_taskdef::{TemporalioActivityDescriptor, WfContext, WfExitValue, WorkflowResult};
use hoover3_tracing::tracing::info;
use hoover3_types::filesystem::ProcessingPlanResult;
use hoover3_types::identifier::CollectionId;

use super::hash_files_plan::chunk_by_size;
use super::FilesystemScannerQueue;

/// Workflow for computing the processing plan for all blobs.
#[workflow(FilesystemScannerQueue)]
async fn compute_blob_processing_plan(
    ctx: WfContext,
    collection_id: CollectionId,
) -> WorkflowResult<ProcessingPlanResult> {
    let plan_page_ids = do_compute_blob_processing_plan_activity::run(&ctx, collection_id).await?;
    Ok(WfExitValue::Normal(plan_page_ids))
}

/// Compute the plan for processing all blobs. Write plan chunks to database.
/// Returns a list of plan page ids.
#[activity(FilesystemScannerQueue)]
pub async fn do_compute_blob_processing_plan(
    collection_id: CollectionId,
) -> anyhow::Result<ProcessingPlanResult> {
    let mut stream = stream_blob_processing_plan(collection_id.clone()).await?;
    let mut plan_page_id = get_max_plan_page_id(&collection_id).await? + 1;
    let mut new_page_count = 0;
    let mut total_blob_size_bytes = 0;
    let mut total_blob_count = 0;
    let db_extra = DatabaseExtraCallbacks::new(&collection_id).await?;
    let db_session = ScyllaDatabaseHandle::collection_session(&collection_id).await?;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        if chunk.blob_hashes.is_empty() {
            continue;
        }

        // Create the plan entry
        let mut plan = BlobProcessingPlan {
            plan_page_id,
            file_count: chunk.blob_hashes.len() as i32,
            size_bytes: chunk.chunk_size,
            is_finished: false,
        };
        total_blob_size_bytes += chunk.chunk_size as u64;
        total_blob_count += chunk.blob_hashes.len() as u32;

        // Insert the plan
        BlobProcessingPlan::insert_cb(&mut plan, &db_extra)
            .execute(&db_session)
            .await?;

        let mut plan_pages = Vec::new();
        let mut partial_updates = Vec::new();

        for item in &chunk.blob_hashes {
            plan_pages.push(BlobProcessingPlanPage {
                plan_page_id,
                blob_sha3_256: item.clone(),
            });
            partial_updates.push(PartialUpdateFsBlobHashesDbRow {
                blob_sha3_256: item.clone(),
                plan_page: Some(plan_page_id),
            });
        }
        BlobProcessingPlanPage::batch()
            .append_inserts(&plan_pages)
            .execute(&db_session)
            .await?;
        PartialUpdateFsBlobHashesDbRow::batch()
            .append_inserts(&partial_updates)
            .execute(&db_session)
            .await?;
        db_extra.insert(&plan_pages).await?;

        new_page_count += 1;
        info!(
            "BLOB PROCESSING PLAN PAGE {}: {} blobs, {} bytes",
            plan_page_id,
            chunk.blob_hashes.len(),
            chunk.chunk_size
        );
        plan_page_id += 1;
    }

    Ok(ProcessingPlanResult {
        new_page_count,
        total_blob_count,
        total_blob_size_bytes,
    })
}

/// Get the maximum existing plan page ID from the database.
/// Returns 0 if no plan pages exist.
async fn get_max_plan_page_id(collection_id: &CollectionId) -> anyhow::Result<i32> {
    let db_session = ScyllaDatabaseHandle::collection_session(collection_id).await?;

    let plans = BlobProcessingPlan::find_all().execute(&db_session).await?;
    pin_mut!(plans);

    let mut max_id = 0;
    while let Some(plan) = plans.next().await {
        let plan = plan?;
        if plan.plan_page_id > max_id {
            max_id = plan.plan_page_id;
        }
    }
    Ok(max_id)
}

fn flatten_result<T, E>(r: Result<Result<T, E>, E>) -> Result<T, E> {
    match r {
        Ok(Ok(r)) => Ok(r),
        Ok(Err(e)) => Err(e),
        Err(e) => Err(e),
    }
}

/// Stream chunks of work for processing blobs.
async fn stream_blob_processing_plan(
    collection_id: CollectionId,
) -> anyhow::Result<ResultStream<BlobProcessingPlanChunk>> {
    let min_read_size = 2_i64.pow(13); // 8 KB
    let max_chunk_size = 2_i64.pow(26); // 64 MB

    // Query all blob hashes
    let db_session = ScyllaDatabaseHandle::collection_session(&collection_id).await?;
    let stream = FsBlobHashesDbRow::find_all().execute(&db_session).await?;

    let stream = async_stream::try_stream! {
        for await r in stream {
           match r {
                Ok(r) => {
                    if r.plan_page.is_none() {
                        yield Ok(r);
                    }
                }
                Err(e) => {
                    yield Err(anyhow::anyhow!("Error in stream_blob_processing_plan: {}", e));
                }
            }
        }
    };

    let stream = stream.map(flatten_result).boxed();

    let stream = reorder_stream(stream, |blob: &FsBlobHashesDbRow| blob.size_bytes, 1000);

    // Chunk by size
    let stream = chunk_by_size(
        stream,
        min_read_size,
        max_chunk_size,
        |blob: &FsBlobHashesDbRow| blob.size_bytes,
    )?;

    // Transform into plan chunks
    let stream = stream.map_ok(|chunk| {
        let mut chunk_size = 0;
        let mut blob_hashes = Vec::with_capacity(chunk.len());

        for blob in chunk {
            blob_hashes.push(blob.blob_sha3_256.clone());
            chunk_size += blob.size_bytes;
        }

        BlobProcessingPlanChunk {
            blob_hashes,
            chunk_size,
        }
    });

    Ok(stream.boxed())
}

struct BlobProcessingPlanChunk {
    blob_hashes: Vec<String>,
    chunk_size: i64,
}

fn reorder_stream<Value, SortKey>(
    stream: ResultStream<Value>,
    sort_key: impl Fn(&Value) -> SortKey + Send + Sync + 'static,
    buffer_size: usize,
) -> ResultStream<Value>
where
    Value: Send + 'static,
    SortKey: Ord + Eq + Send + 'static,
{
    struct OrdItem<SortKey, Value>(SortKey, Value);

    impl<SortKey: Ord + Eq, Value> PartialOrd for OrdItem<SortKey, Value> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.0.partial_cmp(&other.0).map(|o| o.reverse())
        }
    }
    impl<SortKey: Ord + Eq, Value> Ord for OrdItem<SortKey, Value> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0.cmp(&other.0).reverse()
        }
    }
    impl<SortKey: Ord + Eq, Value> PartialEq for OrdItem<SortKey, Value> {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    impl<SortKey: Ord + Eq, Value> Eq for OrdItem<SortKey, Value> {}

    let s = try_stream! {
        let mut buffer = std::collections::BinaryHeap::new();

        let stream = stream.map_ok(|item| {
            let key = sort_key(&item);
            OrdItem(key, item)
        });
        pin_mut!(stream);
        while let Some(item) = stream.next().await {
            let item = item.context("reorder stream error:")?;
            buffer.push(item);
            if buffer.len() > buffer_size {
                let item = buffer.pop().unwrap();
                yield item.1;
            }
        }
        while let Some(item) = buffer.pop() {
            yield item.1;
        }
    };
    Box::pin(s)
}
