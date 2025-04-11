//!
use hoover3_macro::workflow;
use hoover3_taskdef::{
    declare_task_queue, TemporalioWorkflowDescriptor, WfContext, WfExitValue, WorkflowResult,
};
use hoover3_types::{
    identifier::{CollectionId, DatabaseIdentifier},
    processing::ProcessDatasourceTaskResult,
};
declare_task_queue!(ServerTaskQueue, "server_task_queue", 4, 4, 256);

/// Create, scan and process a data source. Uses the "scan" and "process" plugins.
#[workflow(ServerTaskQueue)]
async fn process_datasource(
    wf_ctx: WfContext,
    (collection_id, datasource_id): (CollectionId, DatabaseIdentifier),
) -> WorkflowResult<ProcessDatasourceTaskResult> {
    let scan = hoover3_filesystem_scanner::tasks::scan_filesystem::fs_scan_datasource_workflow::run_as_child(&wf_ctx, (collection_id.clone(), datasource_id.clone())).await?;
    let process = hoover3_processing::tasks::run_collection_processing_workflow::run_as_child(
        &wf_ctx,
        collection_id.clone(),
    )
    .await?;
    Ok(WfExitValue::Normal(ProcessDatasourceTaskResult {
        collection_id,
        datasource_id,
        scan,
        process,
    }))
}
