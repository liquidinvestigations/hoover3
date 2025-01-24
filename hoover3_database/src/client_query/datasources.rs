use crate::db_management::redis::drop_redis_cache;
use crate::db_management::redis::with_redis_cache;
use crate::db_management::redis::with_redis_lock;
use crate::db_management::DatabaseSpaceManager;
use crate::db_management::ScyllaDatabaseHandle;
use crate::models::collection::datasource::DatasourceDbRow;

use anyhow::Result;
use charybdis::operations::{Find, Insert};
use futures::pin_mut;
use hoover3_types::datasource::DatasourceSettings;
use hoover3_types::datasource::DatasourceUiRow;

use hoover3_types::identifier::CollectionId;
use hoover3_types::identifier::DatabaseIdentifier;

use tracing::info;

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

pub async fn create_datasource(
    (c, name, settings): (CollectionId, DatabaseIdentifier, DatasourceSettings),
) -> Result<DatasourceUiRow> {
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
        let row = DatasourceDbRow {
            datasource_id: name.to_string(),
            datasource_type: settings.type_str(),
            datasource_settings: settings_serialized,
            time_created: now,
            time_modified: now,
        };

        DatasourceDbRow::insert(&row).execute(&session).await?;
        Ok(row.to_ui_row(&c))
    })
    .await?
}

pub async fn drop_datasource((c, name): (CollectionId, DatabaseIdentifier)) -> anyhow::Result<()> {
    let session = ScyllaDatabaseHandle::collection_session(&c).await?;
    DatasourceDbRow::delete_by_datasource_id(name.to_string())
        .execute(&session)
        .await?;

    drop_redis_cache("get_datasource", &(c, name)).await?;
    Ok(())
}

pub async fn get_datasource(
    (c, name): (CollectionId, DatabaseIdentifier),
) -> Result<DatasourceUiRow> {
    with_redis_cache(
        "get_datasource",
        3600,
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
