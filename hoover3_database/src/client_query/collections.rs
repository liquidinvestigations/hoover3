use crate::db_management::redis::drop_redis_cache;
use crate::db_management::redis::with_redis_cache;
use crate::db_management::DatabaseSpaceManager;
use crate::db_management::ScyllaDatabaseHandle;
use crate::migrate::migrate_collection;
use crate::models::common::collection::CollectionDbRow;
use anyhow::Result;
use charybdis::operations::{Find, Insert};
use hoover3_types::collection::CollectionUiRow;
use hoover3_types::identifier::CollectionId;
use tracing::info;

/// Client API method, returns details for a single collection in the system.
pub async fn get_single_collection(c: CollectionId) -> Result<CollectionUiRow> {
    let session = ScyllaDatabaseHandle::global_session().await?;
    Ok(CollectionDbRow::find_by_collection_id(c.name())
        .execute(&session)
        .await?
        .into())
}

/// Client API method to create a new collection, including all the
/// databases, tables, search indexes and other setup that must happen.
/// If a collection with said name exists, return that instead.
pub async fn create_new_collection(c: CollectionId) -> Result<CollectionUiRow> {
    info!("creating new collection... {:?}", c);
    let session = ScyllaDatabaseHandle::global_session().await?;

    if let Ok(x) = CollectionDbRow::find_by_collection_id(c.name())
        .execute(&session)
        .await
    {
        return Ok(x.into());
    }
    let now = chrono::offset::Utc::now();
    let new_row = CollectionDbRow {
        collection_id: c.name(),
        collection_title: c.name().replace("_", " "),
        collection_description: "".to_string(),
        time_created: now,
        time_modified: now,
    };
    migrate_collection(&c).await?;
    CollectionDbRow::insert(&new_row).execute(&session).await?;
    drop_redis_cache("get_all_collections", &()).await?;
    Ok(new_row.into())
}

/// Client API method, returns list of all collections in system.
/// Cached for 1min. Cache gets dumped on CREATE, DELETE, MODIFY.
pub async fn get_all_collections(c: ()) -> Result<Vec<CollectionUiRow>, String> {
    with_redis_cache(
        "get_all_collections",
        60,
        move |c| async move {
            _get_all_collections(c)
                .await
                .map_err(|e| format!("_get_all_collections: {e}"))
        },
        &c,
    )
    .await
    .map_err(|e| format!("with_redis_cache: {e}"))?
}

async fn _get_all_collections(_c: ()) -> Result<Vec<CollectionUiRow>> {
    let mut v = vec![];
    let session = ScyllaDatabaseHandle::global_session().await?;
    let mut collections_stream: charybdis::stream::CharybdisModelStream<CollectionDbRow> =
        CollectionDbRow::find_all().execute(&session).await?;
    use futures::StreamExt;
    while let Some(Ok(c)) = collections_stream.next().await {
        v.push(c.into());
    }
    Ok(v)
}

/// Client API method used to update collection title and description.
pub async fn update_collection(updated: CollectionUiRow) -> Result<CollectionUiRow> {
    let session = ScyllaDatabaseHandle::global_session().await?;
    info!("updating collection with user request {:?}", updated);
    let old_row = CollectionDbRow::find_by_collection_id(updated.collection_id)
        .execute(&session)
        .await?;

    info!("updating collection found old {:?}", old_row);
    let now = chrono::offset::Utc::now();
    let new_row = CollectionDbRow {
        collection_id: old_row.collection_id,
        collection_title: updated.collection_title,
        collection_description: updated.collection_description,
        time_created: old_row.time_created,
        time_modified: now,
    };

    info!("updating collection inserting new row {:?}", new_row);
    CollectionDbRow::insert(&new_row).execute(&session).await?;
    drop_redis_cache("get_all_collections", &()).await?;
    info!("updating collection {:?}: done", new_row.collection_id);
    Ok(new_row.into())
}

/// Client API method used to drop a collection, all databases and entries.
pub async fn drop_collection(c: CollectionId) -> Result<()> {
    crate::migrate::drop_collection(&c).await?;

    let session = ScyllaDatabaseHandle::global_session().await?;
    CollectionDbRow::delete_by_collection_id(c.name())
        .execute(&session)
        .await?;

    drop_redis_cache("get_all_collections", &()).await?;
    Ok(())
}

#[tokio::test]
async fn test_collection_query() -> Result<()> {
    // make sure we have common migrations on
    crate::migrate::migrate_common().await?;

    // check we can read collections at all
    get_all_collections(()).await.unwrap();

    // check we can create collections
    let cid = CollectionId::new("test_collection_query")?;
    let mut z = create_new_collection(cid.clone()).await?;
    assert_eq!(z.collection_id, cid.name());

    // check create x2 is ok
    let z1 = create_new_collection(cid.clone()).await?;
    assert_eq!(z.collection_description, z1.collection_description);
    assert_eq!(z.collection_id, z1.collection_id);
    assert_eq!(z.collection_title, z1.collection_title);

    // timestamps are not exact, we lose sub-millisecond in scylla rounding
    assert!((z.time_created.timestamp_millis() - z1.time_created.timestamp_millis()).abs() < 2);
    assert!((z.time_modified.timestamp_millis() - z1.time_modified.timestamp_millis()).abs() < 2);

    // check new collection is in collection list
    assert!(get_all_collections(())
        .await
        .unwrap()
        .into_iter()
        .map(|x| x.collection_id)
        .collect::<Vec<_>>()
        .contains(&z.collection_id));

    // update collection
    z.collection_description = "XXX".to_string();
    z = update_collection(z).await?;
    let z2 = z.clone();
    assert_eq!(z2.collection_id, z1.collection_id);
    assert_eq!(z2.time_created, z1.time_created);
    assert_eq!(z2.collection_description, "XXX".to_string());

    // drop and check it's missing now
    drop_collection(cid.clone()).await?;
    assert!(!get_all_collections(())
        .await
        .unwrap()
        .into_iter()
        .map(|x| x.collection_id)
        .collect::<Vec<_>>()
        .contains(&z.collection_id));
    // check drop x2 is OK
    drop_collection(cid.clone()).await?;

    Ok(())
}
