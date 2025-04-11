//! Server API functions

use hoover3_taskdef::anyhow;
use hoover3_taskdef::TemporalioWorkflowDescriptor;
use hoover3_types::identifier::CollectionId;
use hoover3_types::identifier::DatabaseIdentifier;
use hoover3_types::processing::ProcessDatasourceTaskResult;
use hoover3_types::tasks::UiWorkflowStatus;

use crate::tasks::process_datasource_workflow;

/// API method to get the current memory usage and limit for the server process, in MB
pub async fn get_server_memory_usage(_: ()) -> anyhow::Result<(u32, u32)> {
    Ok((
        hoover3_tracing::get_process_memory_usage(),
        hoover3_tracing::get_process_memory_limit(),
    ))
}

/// Initiates a filesystem scan operation
pub async fn start_processing(
    (c_id, ds_id): (CollectionId, DatabaseIdentifier),
) -> Result<String, anyhow::Error> {
    let w = process_datasource_workflow::client_start(&(c_id, ds_id)).await?;
    Ok(w)
}

/// Waits for and returns filesystem scan results
pub async fn wait_for_processing_results(
    (c_id, ds_id): (CollectionId, DatabaseIdentifier),
) -> Result<ProcessDatasourceTaskResult, anyhow::Error> {
    process_datasource_workflow::client_wait_for_completion(&(c_id, ds_id)).await
}

/// Retrieves current filesystem scan status
pub async fn get_processing_status(
    (c_id, ds_id): (CollectionId, DatabaseIdentifier),
) -> Result<UiWorkflowStatus, anyhow::Error> {
    process_datasource_workflow::client_get_status(&(c_id, ds_id)).await
}
