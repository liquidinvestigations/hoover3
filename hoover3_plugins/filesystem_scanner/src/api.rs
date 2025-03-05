//! Filesystem scanner API - control tasks defined in this crate

use hoover3_taskdef::anyhow;
use hoover3_taskdef::TemporalioWorkflowDescriptor;
use hoover3_types::filesystem::FsScanDatasourceResult;
use hoover3_types::identifier::CollectionId;
use hoover3_types::identifier::DatabaseIdentifier;
use hoover3_types::tasks::UiWorkflowStatus;

use crate::tasks::scan_filesystem::fs_scan_datasource_workflow;
use crate::tasks::scan_filesystem::ScanDatasourceArgs;

/// Initiates a filesystem scan operation
pub async fn start_scan(
    (c_id, ds_id): (CollectionId, DatabaseIdentifier),
) -> Result<String, anyhow::Error> {
    let args = ScanDatasourceArgs {
        collection_id: c_id.clone(),
        datasource_id: ds_id.clone(),
        path: None,
    };
    let w = fs_scan_datasource_workflow::client_start(&args).await?;
    Ok(w)
}

/// Waits for and returns filesystem scan results
pub async fn wait_for_scan_results(
    (c_id, ds_id): (CollectionId, DatabaseIdentifier),
) -> Result<FsScanDatasourceResult, anyhow::Error> {
    let args = ScanDatasourceArgs {
        collection_id: c_id.clone(),
        datasource_id: ds_id.clone(),
        path: None,
    };
    fs_scan_datasource_workflow::client_wait_for_completion(&args).await
}

/// Retrieves current filesystem scan status
pub async fn get_scan_status(
    (c_id, ds_id): (CollectionId, DatabaseIdentifier),
) -> Result<UiWorkflowStatus, anyhow::Error> {
    let args = ScanDatasourceArgs {
        collection_id: c_id.clone(),
        datasource_id: ds_id.clone(),
        path: None,
    };
    fs_scan_datasource_workflow::client_get_status(&args).await
}
