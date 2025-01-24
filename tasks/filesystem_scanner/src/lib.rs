use std::path::PathBuf;
use hoover3_types::datasource::{DatasourceSettings, DatasourceUiRow};
use hoover3_types::identifier::CollectionId;
use hoover3_types::identifier::DatabaseIdentifier;
use hoover3_database::models::collection::filesystem::FsDirectoryDbRow;
use hoover3_database::client_query;
use hoover3_database::models::collection::filesystem::FsFileDbRow;
use hoover3_taskdef::{
    anyhow, make_activity, make_activity_sync, make_workflow, WfContext, WfExitValue,
    WorkflowResult, TemporalioActivityDescriptor
};
use hoover3_database::charybdis::batch::ModelBatch;
use hoover3_database::db_management::DatabaseSpaceManager;
use hoover3_database::db_management::ScyllaDatabaseHandle;
use hoover3_taskdef::TemporalioWorkflowDescriptor;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ScanDatasourceArgs {
    pub collection_id: CollectionId,
    pub datasource_id: DatabaseIdentifier,
    pub datasource_settings: DatasourceSettings,
    pub path: Option<PathBuf>,
}

pub type AllTasks = (
    fs_scan_datasource_workflow,
    fs_do_scan_datasource_activity,
);

pub async fn start_scan((c_id, ds_id): (CollectionId, DatabaseIdentifier)) -> Result<(), anyhow::Error> {
    let args = ScanDatasourceArgs {
        collection_id: c_id.clone(),
        datasource_id: ds_id.clone(),
        datasource_settings: client_query::datasources::get_datasource((c_id.clone(), ds_id.clone())).await?.datasource_settings,
        path: None,
    };
    fs_scan_datasource_workflow::client_start(&args).await?;
    Ok(())
}


make_workflow!(
    fs_scan_datasource,
    ScanDatasourceArgs,
    ()
);
async fn fs_scan_datasource(
    wf_ctx: WfContext,
    args: ScanDatasourceArgs,
) -> WorkflowResult<()> {

    fs_do_scan_datasource_activity::run(&wf_ctx, args).await?;

    Ok(WfExitValue::Normal(()))
}


make_activity!(
    fs_do_scan_datasource,
    ScanDatasourceArgs,
    ()
);
async fn fs_do_scan_datasource(
    arg:ScanDatasourceArgs,
) -> Result<(), anyhow::Error> {
    let DatasourceSettings::LocalDisk{path: root_path} = &arg.datasource_settings else {
        anyhow::bail!("Datasource is not a local disk");
    };
    let dir_path = root_path.to_path_buf().join((&arg.path).clone().unwrap_or(PathBuf::from("")));
    let children = client_query::list_disk::list_directory(dir_path).await?;
    let mut files = vec![];
    let mut dirs = vec![];

    children.into_iter().for_each(|mut c| {
        c.path = c.path.strip_prefix(&root_path).unwrap().to_path_buf();
        if c.is_file {
            files.push(FsFileDbRow::from_meta(&arg.datasource_id, &c));
        } else if c.is_dir {
            dirs.push(FsDirectoryDbRow::from_meta(&arg.datasource_id, &c));
        }
    });

    let session = ScyllaDatabaseHandle::collection_session(&arg.collection_id)
    .await?;
    FsFileDbRow::batch().chunked_insert(&session, &files, 1024).await?;
    FsDirectoryDbRow::batch().chunked_insert(&session, &dirs, 1024).await?;

    // if all is OK, spawn client jobs
    for dir in dirs.into_iter()  {
        let arg2 = arg.clone();
        let new_arg = ScanDatasourceArgs {
            path: Some(dir.path.clone().into()),
            ..arg2
        };
        fs_scan_datasource_workflow::client_start(&new_arg).await?;
    }

    Ok(())
}
