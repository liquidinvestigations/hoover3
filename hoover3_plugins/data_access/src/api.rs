//! Datasource management module that provides functionality to create, update, and delete datasources.

use charybdis::operations::InsertWithCallbacks;
use hoover3_database::db_management::redis::drop_redis_cache;
use hoover3_database::db_management::redis::with_redis_cache;
use hoover3_database::db_management::redis::with_redis_lock;
use hoover3_database::db_management::DatabaseSpaceManager;
use hoover3_database::db_management::ScyllaDatabaseHandle;
use hoover3_database::models::collection::DatabaseExtraCallbacks;

use anyhow::Result;
use charybdis::operations::Find;
use futures::pin_mut;
use hoover3_types::datasource::DatasourceSettings;
use hoover3_types::datasource::DatasourceUiRow;

use hoover3_types::identifier::CollectionId;
use hoover3_types::identifier::DatabaseIdentifier;

use tracing::info;

use crate::models::DatasourceDbRow;

/// Client API method, returns details for a single collection in the system.
pub async fn get_all_datasources(c: CollectionId) -> Result<Vec<DatasourceUiRow>> {
    let session = ScyllaDatabaseHandle::collection_session(&c).await?;
    let rows = DatasourceDbRow::find_all().execute(&session).await?;
    pin_mut!(rows);
    let mut v = vec![];
    use futures::StreamExt;
    while let Some(Ok(x)) = rows.next().await {
        v.push(x.to_ui_row(&c));
    }
    Ok(v)
}

/// Create a new datasource in the given collection. Repeated calls using the same name will return the same datasource.
/// A tokio spawn task is used to ensure the creation is finalized even if the user disconnects.
pub async fn create_datasource(
    (c, name, settings): (CollectionId, DatabaseIdentifier, DatasourceSettings),
) -> Result<DatasourceUiRow> {
    tokio::spawn(async move {
        info!("create_datasource collection={c:?} datasource={name:?} settings={settings:?}");
        with_redis_lock("create_datasource", async move {
            let session = ScyllaDatabaseHandle::collection_session(&c).await?;
            let settings_serialized = serde_json::to_string(&settings)?;
            if let Ok(ds) = DatasourceDbRow::find_by_datasource_id(name.to_string())
                .execute(&session)
                .await
            {
                return Ok(ds.to_ui_row(&c));
            }
            let now = chrono::offset::Utc::now();
            let mut row = DatasourceDbRow {
                datasource_id: name.to_string(),
                datasource_type: settings.type_str(),
                datasource_settings: settings_serialized,
                time_created: now,
                time_modified: now,
            };
            let cb_info = DatabaseExtraCallbacks::new(&c).await?;
            DatasourceDbRow::insert_cb(&mut row, &cb_info)
                .execute(&session)
                .await?;
            Ok(row.to_ui_row(&c))
        })
        .await?
    })
    .await?
}

/// Drop a datasource from the given collection. Dropping a non-existent datasource is a no-op.
pub async fn drop_datasource((c, name): (CollectionId, DatabaseIdentifier)) -> anyhow::Result<()> {
    tokio::spawn(async move {
        let session = ScyllaDatabaseHandle::collection_session(&c).await?;
        DatasourceDbRow::delete_by_datasource_id(name.to_string())
            .execute(&session)
            .await?;

        drop_redis_cache("get_datasource", &(c, name)).await?;
        Ok(())
    })
    .await?
}

/// Get details for a single datasource in the given collection.
pub async fn get_datasource(
    (c, name): (CollectionId, DatabaseIdentifier),
) -> Result<DatasourceUiRow> {
    with_redis_cache(
        "get_datasource",
        60,
        move |(c, name)| async move {
            let session = ScyllaDatabaseHandle::collection_session(&c).await?;
            let row = DatasourceDbRow::find_by_datasource_id(name.to_string())
                .execute(&session)
                .await?;
            Ok(row.to_ui_row(&c))
        },
        &(c, name),
    )
    .await
}

#[tokio::test]
async fn test_datasource_query() -> Result<()> {
    // make sure we have common migrations on
    hoover3_database::migrate::migrate_common().await?;

    use hoover3_database::client_query::collections::create_new_collection;
    use hoover3_database::client_query::collections::drop_collection;
    use hoover3_database::client_query::collections::get_all_collections;
    use std::path::PathBuf;

    // check we can read collections at all
    get_all_collections(()).await.unwrap();

    // check we can create collections
    let cid = CollectionId::new("test_datasource_query")?;
    create_new_collection(cid.clone()).await?;

    let name = DatabaseIdentifier::new("test_datasource_query")?;
    let settings = DatasourceSettings::LocalDisk {
        path: PathBuf::from("hoover-testdata/eml-1-promotional"),
    };
    create_datasource((cid.clone(), name.clone(), settings.clone())).await?;

    let ds = get_datasource((cid.clone(), name.clone())).await?;
    assert_eq!(ds.datasource_id.to_string(), name.to_string());

    let list = get_all_datasources(cid.clone())
        .await?
        .into_iter()
        .map(|x| x.datasource_id)
        .collect::<Vec<_>>();
    assert!(list.contains(&name));

    drop_datasource((cid.clone(), name.clone())).await?;

    let list = get_all_datasources(cid.clone())
        .await?
        .into_iter()
        .map(|x| x.datasource_id)
        .collect::<Vec<_>>();
    assert!(!list.contains(&name));

    drop_collection(cid.clone()).await?;

    Ok(())
}
