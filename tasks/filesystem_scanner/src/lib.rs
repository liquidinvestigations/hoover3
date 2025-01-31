use hoover3_database::charybdis::batch::ModelBatch;
use hoover3_database::client_query;
use hoover3_database::db_management::DatabaseSpaceManager;
use hoover3_database::db_management::ScyllaDatabaseHandle;
use hoover3_database::models::collection::filesystem::FsDirectoryDbRow;
use hoover3_database::models::collection::filesystem::FsFileDbRow;
use hoover3_taskdef::TemporalioWorkflowDescriptor;
use hoover3_taskdef::{
    activity, anyhow, workflow, TemporalioActivityDescriptor, WfContext, WfExitValue,
    WorkflowResult,
};
use hoover3_types::datasource::DatasourceSettings;
use hoover3_types::filesystem::FsScanDatasourceResult;
use hoover3_types::identifier::CollectionId;
use hoover3_types::identifier::DatabaseIdentifier;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const FILESYSTEM_SCANNER_TASK_QUEUE: &str = "filesystem_scanner";

#[derive(Serialize, Deserialize, Clone)]
pub struct ScanDatasourceArgs {
    pub collection_id: CollectionId,
    pub datasource_id: DatabaseIdentifier,
    pub path: Option<PathBuf>,
}

pub type AllTasks = (
    fs_scan_datasource_workflow,
    fs_do_scan_datasource_activity,
    fs_scan_datasource_group_workflow,
);

pub async fn start_scan(
    (c_id, ds_id): (CollectionId, DatabaseIdentifier),
) -> Result<(), anyhow::Error> {
    let args = ScanDatasourceArgs {
        collection_id: c_id.clone(),
        datasource_id: ds_id.clone(),
        path: None,
    };
    fs_scan_datasource_workflow::client_start(&args).await?;
    Ok(())
}

#[workflow(FILESYSTEM_SCANNER_TASK_QUEUE)]
async fn fs_scan_datasource(
    wf_ctx: WfContext,
    args: ScanDatasourceArgs,
) -> WorkflowResult<FsScanDatasourceResult> {
    let (mut scan_result, next_paths) =
        fs_do_scan_datasource_activity::run(&wf_ctx, args.clone()).await?;

    let args = next_paths
        .into_iter()
        .map(|p| ScanDatasourceArgs {
            collection_id: args.collection_id.clone(),
            datasource_id: args.datasource_id.clone(),
            path: Some(p),
        })
        .collect::<Vec<_>>();

    let results = if args.len() < 10 {
        fs_scan_datasource_workflow::run_parallel(&wf_ctx, args)
            .await?
            .into_iter()
            .map(|r| r.1)
            .collect::<Vec<_>>()
    } else {
        // to avoid large workflow history, break this into smaller chunks
        let chunk_size = ((1.0 + args.len() as f64).sqrt()).ceil() as usize;
        let groups = args
            .chunks(chunk_size)
            .map(|c| c.to_vec())
            .collect::<Vec<_>>();
        fs_scan_datasource_group_workflow::run_parallel(&wf_ctx, groups)
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
    Ok(WfExitValue::Normal(scan_result))
}

#[workflow(FILESYSTEM_SCANNER_TASK_QUEUE)]
async fn fs_scan_datasource_group(
    wf_ctx: WfContext,
    args: Vec<ScanDatasourceArgs>,
) -> WorkflowResult<FsScanDatasourceResult> {
    let results = fs_scan_datasource_workflow::run_parallel(&wf_ctx, args).await?;
    let mut scan_result = FsScanDatasourceResult::default();
    for (_, r) in results {
        if let Ok(r) = r {
            scan_result += r;
        } else {
            scan_result.errors += 1;
        }
    }
    Ok(WfExitValue::Normal(scan_result))
}

#[activity(FILESYSTEM_SCANNER_TASK_QUEUE)]
async fn fs_do_scan_datasource(
    arg: ScanDatasourceArgs,
) -> anyhow::Result<(FsScanDatasourceResult, Vec<PathBuf>)> {
    let mut file_count = 0;
    let mut dir_count = 0;
    let mut file_size_bytes = 0;
    let ds_row = client_query::datasources::get_datasource((
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
    let children = client_query::list_disk::list_directory(dir_path).await?;
    let mut files = vec![];
    let mut dirs = vec![];
    let mut next_paths = vec![];

    children.into_iter().for_each(|mut c| {
        c.path = c.path.strip_prefix(root_path).unwrap().to_path_buf();
        if c.is_file {
            files.push(FsFileDbRow::from_meta(&arg.datasource_id, &c));
            file_count += 1;
            file_size_bytes += c.size_bytes;
        } else if c.is_dir {
            dirs.push(FsDirectoryDbRow::from_meta(&arg.datasource_id, &c));
            dir_count += 1;
            next_paths.push(c.path.clone());
        }
    });

    let session = ScyllaDatabaseHandle::collection_session(&arg.collection_id).await?;
    FsFileDbRow::batch()
        .chunked_insert(&session, &files, 1024)
        .await?;
    FsDirectoryDbRow::batch()
        .chunked_insert(&session, &dirs, 1024)
        .await?;
    let db_extra = hoover3_database::models::collection::DatabaseExtraCallbacks::new(&arg.collection_id.clone()).await?;
    db_extra.insert(&files).await?;
    db_extra.insert(&dirs).await?;

    next_paths.sort();
    next_paths.dedup();

    Ok((
        FsScanDatasourceResult {
            file_count,
            dir_count,
            file_size_bytes,
            errors: 0,
        },
        next_paths,
    ))
}
