//! Test the filesystem scanner API
use std::path::PathBuf;

use hoover3_database::migrate::migrate_common;
use hoover3_filesystem_scanner::{api::{get_scan_status, start_scan, wait_for_scan_results}, tasks::AllTasks};
use hoover3_types::{datasource::DatasourceSettings, identifier::{CollectionId, DatabaseIdentifier}};

use hoover3_database::client_query::collections::{create_new_collection, drop_collection};
use hoover3_data_access::api::{create_datasource, get_all_datasources};
use hoover3_types::tasks::UiWorkflowStatusCode;

#[tokio::test]
async fn test_fs_do_scan_datasource_small() -> anyhow::Result<()> {
    migrate_common().await?;
    let collection_id = CollectionId::new("test_fs_do_scan_datasource")?;
    drop_collection(collection_id.clone()).await?;
    create_new_collection(collection_id.clone()).await?;
    assert!(
    get_all_datasources(collection_id.clone())
            .await?
            .is_empty()
    );
    let datasource_id = DatabaseIdentifier::new("test_fs_do_scan_datasource_collection")?;
    let settings = DatasourceSettings::LocalDisk {
        path: PathBuf::from("hoover-testdata/data/disk-files/long-filenames"),
    };
    create_datasource((
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
    drop_collection(collection_id.clone()).await?;
    Ok(())
}
