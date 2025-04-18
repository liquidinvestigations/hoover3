use futures::{pin_mut, StreamExt};
use hoover3_database::{
    charybdis::operations::Find,
    db_management::{DatabaseSpaceManager, ScyllaDatabaseHandle},
};
use hoover3_filesystem_scanner::models::BlobProcessingPlan;
use hoover3_taskdef::{
    activity, anyhow, workflow, TemporalioActivityDescriptor, TemporalioWorkflowDescriptor,
    WfContext, WfExitValue, WorkflowResult,
};
use hoover3_types::{identifier::CollectionId, processing::ProcessPageResult};
use serde::{Deserialize, Serialize};

use super::{
    process_page::{process_big_page_activity, process_small_page_activity},
    ProcessingTasksQueue,
};

const SMALL_THRESHOLD: i64 = 100 * 1024 * 1024; // 100MB

/// Activity for fetching the plan pages for a collection.
#[activity(ProcessingTasksQueue)]
async fn get_plan_page_ids(collection_id: CollectionId) -> anyhow::Result<(Vec<i32>, Vec<i32>)> {
    let session = ScyllaDatabaseHandle::collection_session(&collection_id).await?;
    let page_stream = BlobProcessingPlan::find_all().execute(&session).await?;
    pin_mut!(page_stream);
    let mut small_pages = vec![];
    let mut large_pages = vec![];
    while let Some(page) = page_stream.next().await {
        let page = page?;
        if page.is_started {
            continue;
        }
        if page.size_bytes < SMALL_THRESHOLD {
            small_pages.push(page.plan_page_id);
        } else {
            large_pages.push(page.plan_page_id);
        }
    }

    Ok((small_pages, large_pages))
}

/// Arguments for processing a single page.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProcessPageArgs {
    pub collection_id: CollectionId,
    pub plan_page_id: i32,
    pub page_is_small: bool,
}

/// Execute one processing page.
#[workflow(ProcessingTasksQueue)]
async fn process_page_one(
    ctx: WfContext,
    args: ProcessPageArgs,
) -> WorkflowResult<ProcessPageResult> {
    if args.page_is_small {
        Ok(WfExitValue::Normal(
            process_small_page_activity::run(&ctx, args).await?,
        ))
    } else {
        Ok(WfExitValue::Normal(
            process_big_page_activity::run(&ctx, args).await?,
        ))
    }
}

/// Process a group of plan chunks in parallel in parallel.
#[workflow(ProcessingTasksQueue)]
async fn process_pages_group(
    ctx: WfContext,
    (collection_id, pages, is_small): (CollectionId, Vec<i32>, bool),
) -> WorkflowResult<ProcessPageResult> {
    if pages.len() < 300 {
        let args = pages
            .into_iter()
            .map(|plan_page_id| ProcessPageArgs {
                collection_id: collection_id.clone(),
                plan_page_id,
                page_is_small: is_small,
            })
            .collect::<Vec<_>>();

        let mut total = ProcessPageResult::default();
        for (_arg, res) in process_page_one_workflow::run_parallel(&ctx, args).await? {
            let _res = res?;
            total.item_count += _res.item_count;
            total.item_success += _res.item_success;
            total.item_errors += _res.item_errors;
        }
        Ok(WfExitValue::Normal(total))
    } else {
        let chunks = pages
            .chunks(300)
            .map(|v| (collection_id.clone(), v.to_vec(), is_small))
            .collect::<Vec<_>>();
        let mut total = ProcessPageResult::default();
        for (_arg, res) in process_pages_group_workflow::run_parallel(&ctx, chunks).await? {
            let _res = res?;
            total.item_count += _res.item_count;
            total.item_success += _res.item_success;
            total.item_errors += _res.item_errors;
        }
        Ok(WfExitValue::Normal(total))
    }
}
