use hoover3_database::charybdis::batch::ModelBatch;
use hoover3_database::client_query;
use hoover3_database::db_management::DatabaseSpaceManager;
use hoover3_database::db_management::ScyllaDatabaseHandle;
use hoover3_database::models::collection::_nebula_edges::FilesystemParentEdge;
use hoover3_database::models::collection::filesystem::FsDirectoryDbRow;
use hoover3_database::models::collection::filesystem::FsFileDbRow;
use hoover3_database::models::collection::InsertEdgeBatch;
use hoover3_taskdef::TemporalioWorkflowDescriptor;
use hoover3_taskdef::{
    activity, anyhow, workflow, TemporalioActivityDescriptor, WfContext, WfExitValue,
    WorkflowResult,
};
use hoover3_types::datasource::DatasourceSettings;
use hoover3_types::filesystem::FsScanDatasourceResult;
use hoover3_types::identifier::CollectionId;
use hoover3_types::identifier::DatabaseIdentifier;
use hoover3_types::tasks::UiWorkflowStatus;
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
    fs_save_dir_scan_total_result_activity,
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

#[workflow(FILESYSTEM_SCANNER_TASK_QUEUE)]
async fn fs_scan_datasource(
    wf_ctx: WfContext,
    args: ScanDatasourceArgs,
) -> WorkflowResult<FsScanDatasourceResult> {
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
        fs_scan_datasource_workflow::run_parallel(&wf_ctx, next_args)
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

    fs_save_dir_scan_total_result_activity::run(&wf_ctx, (vec![args.clone()], scan_result)).await?;
    Ok(WfExitValue::Normal(scan_result))
}

#[activity(FILESYSTEM_SCANNER_TASK_QUEUE)]
async fn fs_save_dir_scan_total_result(
    (args, scan_result): (Vec<ScanDatasourceArgs>, FsScanDatasourceResult),
) -> anyhow::Result<()> {
    if args.is_empty() {
        return Ok(());
    }
    let collection_id = args[0].collection_id.clone();
    let scylla_session = ScyllaDatabaseHandle::collection_session(&collection_id).await?;
    let mut dirs = vec![];
    for arg in args {
        use hoover3_database::charybdis::operations::Find;
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
    let mut edges_to_files = InsertEdgeBatch::new(FilesystemParentEdge);
    let mut edges_to_dirs = InsertEdgeBatch::new(FilesystemParentEdge);

    let scylla_session = ScyllaDatabaseHandle::collection_session(&arg.collection_id).await?;
    let mut parent_pk = if arg.path.is_some() {
        let p = arg.path.unwrap();
        use hoover3_database::charybdis::operations::Find;
        Some(
            FsDirectoryDbRow::find_by_primary_key_value((
                arg.datasource_id.to_string(),
                p.to_str().unwrap().into(),
            ))
            .execute(&scylla_session)
            .await?,
        )
    } else {
        None
    };
    // let parent_pk = arg.path.map(|p| FsDirectoryDbRow {
    //     datasource_id: arg.datasource_id.to_string(),
    //     path: p.to_str().unwrap().into(),
    //     ..Default::default()
    // });
    children.into_iter().for_each(|mut c| {
        c.path = c.path.strip_prefix(root_path).unwrap().to_path_buf();
        if c.is_file {
            let new_file = FsFileDbRow::from_basic_meta(&arg.datasource_id, &c);
            if let Some(parent_pk) = &parent_pk {
                edges_to_files.push(parent_pk, &new_file);
            }
            files.push(new_file);
            file_count += 1;
            file_size_bytes += c.size_bytes;
        } else if c.is_dir {
            let new_dir = FsDirectoryDbRow::from_basic_meta(&arg.datasource_id, &c);
            if let Some(parent_pk) = &parent_pk {
                edges_to_dirs.push(parent_pk, &new_dir);
            }
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

    let db_extra = hoover3_database::models::collection::DatabaseExtraCallbacks::new(
        &arg.collection_id.clone(),
    )
    .await?;
    db_extra.insert(&files).await?;
    db_extra.insert(&dirs).await?;
    edges_to_files.execute(&db_extra).await?;
    edges_to_dirs.execute(&db_extra).await?;

    next_paths.sort();
    next_paths.dedup();
    if let Some(parent_pk) = &mut parent_pk {
        parent_pk.scan_children.file_count = file_count as i32;
        parent_pk.scan_children.dir_count = dir_count as i32;
        parent_pk.scan_children.file_size_bytes = file_size_bytes as i64;
        parent_pk.scan_children.errors = 0_i32;
        use hoover3_database::charybdis::operations::UpdateWithCallbacks;
        FsDirectoryDbRow::update_cb(parent_pk, &db_extra)
            .execute(&scylla_session)
            .await?;
    }

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

#[tokio::test]
async fn test_fs_do_scan_datasource() -> anyhow::Result<()> {
    hoover3_database::migrate::migrate_common().await?;
    use hoover3_types::tasks::UiWorkflowStatusCode;
    let collection_id = CollectionId::new("test_fs_do_scan_datasource")?;
    use hoover3_database::client_query;
    client_query::collections::drop_collection(collection_id.clone()).await?;
    client_query::collections::create_new_collection(collection_id.clone()).await?;
    assert!(
        client_query::datasources::get_all_datasources(collection_id.clone())
            .await?
            .is_empty()
    );
    let datasource_id = DatabaseIdentifier::new("test_fs_do_scan_datasource_collection")?;
    let settings = DatasourceSettings::LocalDisk {
        path: PathBuf::from("hoover-testdata/data/disk-files/long-filenames"),
    };
    client_query::datasources::create_datasource((
        collection_id.clone(),
        datasource_id.clone(),
        settings,
    ))
    .await?;

    hoover3_taskdef::spawn_worker_on_thread::<AllTasks>();

    start_scan((collection_id.clone(), datasource_id.clone())).await?;
    let status = wait_for_scan_results((collection_id.clone(), datasource_id.clone())).await?;
    assert_eq!(status.file_count, 3);
    assert_eq!(status.dir_count, 0);
    assert_eq!(status.file_size_bytes, 308482);
    assert_eq!(status.errors, 0);
    let status = get_scan_status((collection_id.clone(), datasource_id.clone())).await?;
    assert_eq!(status.task_status, UiWorkflowStatusCode::Completed);
    client_query::collections::drop_collection(collection_id.clone()).await?;
    Ok(())
}
