//! Filesystem scanner - go over data access api and scan that filesystem.
//! Scan results (files and directories) are saved to the database.

use charybdis::batch::ModelBatch;
use charybdis::operations::Find;
use charybdis::operations::InsertWithCallbacks;
use charybdis::operations::UpdateWithCallbacks;
use hoover3_database::client_query::list_disk::get_path_metadata;
use hoover3_database::client_query::list_disk::list_directory;
use hoover3_database::db_management::DatabaseSpaceManager;
use hoover3_database::db_management::ScyllaDatabaseHandle;
use hoover3_taskdef::TemporalioWorkflowDescriptor;
use hoover3_taskdef::{
    activity, anyhow, workflow, TemporalioActivityDescriptor, WfContext, WfExitValue,
    WorkflowResult,
};
use hoover3_types::datasource::DatasourceSettings;
use hoover3_types::filesystem::FsScanDatasourceDirsResult;
use hoover3_types::filesystem::FsScanResult;
use hoover3_types::identifier::CollectionId;
use hoover3_types::identifier::DatabaseIdentifier;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::models::FsDirectoryDbRow;
use crate::models::FsFileDbRow;

use super::hash_files::hash_files_root_workflow;
use super::process_plan::compute_blob_processing_plan_workflow;
use super::FilesystemScannerQueue;

/// Arguments for filesystem datasource scanning
#[derive(Serialize, Deserialize, Clone)]
pub struct ScanDatasourceArgs {
    /// Collection identifier
    pub collection_id: CollectionId,
    /// Datasource identifier
    pub datasource_id: DatabaseIdentifier,
    /// Optional path to scan, defaults to root if None
    pub path: Option<PathBuf>,
}

/// Workflow for scanning a filesystem datasource. Calls child workflows that:
/// - Scan the root directory of datasource
/// - Hash the files in the datasource
#[workflow(FilesystemScannerQueue)]
async fn fs_scan_datasource(
    wf_ctx: WfContext,
    (collection_id, datasource_id): (CollectionId, DatabaseIdentifier),
) -> WorkflowResult<FsScanResult> {
    let _scan_dir = fs_scan_datasource_dir_workflow::run_as_child(
        &wf_ctx,
        ScanDatasourceArgs {
            collection_id: collection_id.clone(),
            datasource_id: datasource_id.clone(),
            path: None,
        },
    )
    .await?;

    let _hash_files = hash_files_root_workflow::run_as_child(
        &wf_ctx,
        (collection_id.clone(), datasource_id.clone()),
    )
    .await?;

    let _process_plan =
        compute_blob_processing_plan_workflow::run_as_child(&wf_ctx, collection_id.clone()).await?;

    Ok(WfExitValue::Normal(FsScanResult {
        dir_scan_result: _scan_dir,
        hash_scan_result: _hash_files,
        processing_plan_result: _process_plan,
    }))
}

/// Workflow for scanning a filesystem datasource
#[workflow(FilesystemScannerQueue)]
async fn fs_scan_datasource_dir(
    wf_ctx: WfContext,
    args: ScanDatasourceArgs,
) -> WorkflowResult<FsScanDatasourceDirsResult> {
    let (mut scan_result, next_paths) =
        fs_do_scan_datasource_activity::run(&wf_ctx, args.clone()).await?;

    let next_args = next_paths
        .into_iter()
        .map(|p| ScanDatasourceArgs {
            collection_id: args.collection_id.clone(),
            datasource_id: args.datasource_id.clone(),
            path: Some(p),
        })
        .collect::<Vec<_>>();

    let results = if next_args.len() < 10 {
        fs_scan_datasource_dir_workflow::run_parallel(&wf_ctx, next_args)
            .await?
            .into_iter()
            .map(|r| r.1)
            .collect::<Vec<_>>()
    } else {
        // to avoid large workflow history, break this into smaller chunks
        let chunk_size = ((1.0 + next_args.len() as f64).sqrt()).ceil() as usize;
        let groups = next_args
            .chunks(chunk_size)
            .map(|c| c.to_vec())
            .collect::<Vec<_>>();
        fs_scan_datasource_dir_group_workflow::run_parallel(&wf_ctx, groups)
            .await?
            .into_iter()
            .map(|r| r.1)
            .collect::<Vec<_>>()
    };

    for r in results {
        if let Ok(r) = r {
            scan_result += r;
        } else {
            scan_result.errors += 1;
        }
    }

    fs_save_dir_scan_total_result_activity::run(&wf_ctx, (vec![args.clone()], scan_result)).await?;
    Ok(WfExitValue::Normal(scan_result))
}

/// Activity for saving directory scan results
#[activity(FilesystemScannerQueue)]
async fn fs_save_dir_scan_total_result(
    (args, scan_result): (Vec<ScanDatasourceArgs>, FsScanDatasourceDirsResult),
) -> anyhow::Result<()> {
    if args.is_empty() {
        return Ok(());
    }
    let collection_id = args[0].collection_id.clone();
    let scylla_session = ScyllaDatabaseHandle::collection_session(&collection_id).await?;
    let mut dirs = vec![];
    for arg in args {
        use charybdis::operations::Find;
        if let Some(path) = arg.path {
            let mut dir = FsDirectoryDbRow::find_by_primary_key_value((
                arg.datasource_id.to_string(),
                path.to_str().unwrap().into(),
            ))
            .execute(&scylla_session)
            .await?;
            dir.scan_total.file_count = scan_result.file_count as i32;
            dir.scan_total.dir_count = scan_result.dir_count as i32;
            dir.scan_total.file_size_bytes = scan_result.file_size_bytes as i64;
            dir.scan_total.errors = scan_result.errors as i32;
            dirs.push(dir);
        }
    }
    if dirs.is_empty() {
        return Ok(());
    }
    FsDirectoryDbRow::batch()
        .chunked_insert(&scylla_session, &dirs, 1024)
        .await?;
    let db_extra =
        hoover3_database::models::collection::DatabaseExtraCallbacks::new(&collection_id).await?;
    db_extra.insert(&dirs).await?;
    Ok(())
}

/// Workflow for processing groups of filesystem scans, used to avoid large workflow history
#[workflow(FilesystemScannerQueue)]
async fn fs_scan_datasource_dir_group(
    wf_ctx: WfContext,
    args: Vec<ScanDatasourceArgs>,
) -> WorkflowResult<FsScanDatasourceDirsResult> {
    let results = fs_scan_datasource_dir_workflow::run_parallel(&wf_ctx, args).await?;
    let mut scan_result = FsScanDatasourceDirsResult::default();
    for (_, r) in results {
        if let Ok(r) = r {
            scan_result += r;
        } else {
            scan_result.errors += 1;
        }
    }
    Ok(WfExitValue::Normal(scan_result))
}

/// Activity for performing filesystem directory scanning
#[activity(FilesystemScannerQueue)]
async fn fs_do_scan_datasource(
    arg: ScanDatasourceArgs,
) -> anyhow::Result<(FsScanDatasourceDirsResult, Vec<PathBuf>)> {
    let mut file_count = 0;
    let mut dir_count = 0;
    let mut file_size_bytes = 0;
    let ds_row = hoover3_data_access::api::get_datasource((
        arg.collection_id.clone(),
        arg.datasource_id.clone(),
    ))
    .await?;

    let DatasourceSettings::LocalDisk { path: root_path } = &ds_row.datasource_settings else {
        anyhow::bail!("Datasource is not a local disk");
    };
    let dir_path = root_path
        .to_path_buf()
        .join(arg.path.clone().unwrap_or(PathBuf::from("")));

    let children = list_directory(dir_path.clone()).await?;
    let mut files = vec![];
    let mut dirs = vec![];
    let mut next_paths = vec![];

    let scylla_session = ScyllaDatabaseHandle::collection_session(&arg.collection_id).await?;

    let db_extra = hoover3_database::models::collection::DatabaseExtraCallbacks::new(
        &arg.collection_id.clone(),
    )
    .await?;

    let mut parent_pk = match arg.path {
        Some(p) => {
            FsDirectoryDbRow::find_by_primary_key_value((
                arg.datasource_id.to_string(),
                p.to_str().unwrap().into(),
            ))
            .execute(&scylla_session)
            .await?
        }
        None => {
            let root_meta = get_path_metadata(PathBuf::from("")).await?;
            let mut dir = FsDirectoryDbRow::from_basic_meta(&arg.datasource_id, &root_meta);
            FsDirectoryDbRow::insert_cb(&mut dir, &db_extra)
                .execute(&scylla_session)
                .await?;
            dir
        }
    };

    children.into_iter().for_each(|mut c| {
        c.path = c.path.strip_prefix(root_path).unwrap().to_path_buf();
        if c.is_file {
            let new_file = FsFileDbRow::from_basic_meta(&arg.datasource_id, &c);

            files.push(new_file);
            file_count += 1;
            file_size_bytes += c.size_bytes;
        } else if c.is_dir {
            let new_dir = FsDirectoryDbRow::from_basic_meta(&arg.datasource_id, &c);

            dirs.push(new_dir);
            dir_count += 1;
            next_paths.push(c.path.clone());
        }
    });

    FsFileDbRow::batch()
        .chunked_insert(&scylla_session, &files, 1024)
        .await?;
    FsDirectoryDbRow::batch()
        .chunked_insert(&scylla_session, &dirs, 1024)
        .await?;

    db_extra.insert(&files).await?;
    db_extra.insert(&dirs).await?;

    next_paths.sort();
    next_paths.dedup();
    parent_pk.scan_children.file_count = file_count as i32;
    parent_pk.scan_children.dir_count = dir_count as i32;
    parent_pk.scan_children.file_size_bytes = file_size_bytes as i64;
    parent_pk.scan_children.errors = 0_i32;
    FsDirectoryDbRow::update_cb(&mut parent_pk, &db_extra)
        .execute(&scylla_session)
        .await?;

    Ok((
        FsScanDatasourceDirsResult {
            file_count,
            dir_count,
            file_size_bytes,
            errors: 0,
        },
        next_paths,
    ))
}
