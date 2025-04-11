//! Task definitions for the processing plugin.

mod get_mime_type;
mod process_group;
mod process_page;

use hoover3_taskdef::{
    declare_task_queue, workflow, TemporalioActivityDescriptor, TemporalioWorkflowDescriptor,
    WfContext, WfExitValue, WorkflowResult,
};
use hoover3_types::{identifier::CollectionId, processing::CollectionProcessingResult};
use process_group::{get_plan_page_ids_activity, process_pages_group_workflow};

declare_task_queue!(
    ProcessingTasksQueue,
    "processing_tasks",
    8,    // concurrent workflows
    32,   // max i/o threads
    2048  // MB ram worker total
);

declare_task_queue!(
    ProcessingQueueSmallPage,
    "processing_small",
    8,    // concurrent workflows
    64,   // max i/o threads
    4096  // MB ram worker total
);

declare_task_queue!(
    ProcessingQueueBigPage,
    "processing_big",
    3,    // concurrent workflows
    16,   // max i/o threads
    4096  // MB ram worker total
);

/// Workflow for computing the processing plan for all blobs.
#[workflow(ProcessingTasksQueue)]
async fn run_collection_processing(
    ctx: WfContext,
    collection_id: CollectionId,
) -> WorkflowResult<CollectionProcessingResult> {
    let (small_pages, large_pages) =
        get_plan_page_ids_activity::run(&ctx, collection_id.clone()).await?;
    let small_page_cnt = small_pages.len() as i32;
    let large_page_cnt = large_pages.len() as i32;

    let _r1 = process_pages_group_workflow::run_as_child(
        &ctx,
        (collection_id.clone(), small_pages, true),
    )
    .await?;
    let _r2 = process_pages_group_workflow::run_as_child(
        &ctx,
        (collection_id.clone(), large_pages, false),
    )
    .await?;

    Ok(WfExitValue::Normal(CollectionProcessingResult {
        collection_id,
        small_page_count: small_page_cnt,
        large_page_count: large_page_cnt,
        small_page_results: _r1,
        large_page_results: _r2,
    }))
}
