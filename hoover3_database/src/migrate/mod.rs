//! This module contains the database migration functions,
//! including mirroring the Scylla schema into other databases.

use hoover3_types::db_schema::CollectionSchemaDynamic;

use anyhow::Context;
use anyhow::Result;
use std::path::PathBuf;
use tracing::info;

use crate::db_management::redis::drop_redis_cache;
use crate::db_management::CollectionId;
use crate::db_management::MeilisearchDatabaseHandle;
use crate::db_management::S3DatabaseHandle;
use crate::{db_management::DatabaseSpaceManager, db_management::ScyllaDatabaseHandle};
use hoover3_types::identifier::DEFAULT_KEYSPACE_NAME;

use super::db_management::redis::with_redis_lock;

/// Sometimes we run from workspace root. Sometimes we run from package root.
/// We need to point to correct migration dirs; so we need to identify this package's dir.
/// This function tries a few variations and finds the `hoover3_database` package root.
pub fn get_package_dir() -> PathBuf {
    let package_name = "hoover3_database";
    let p = std::env::current_dir().unwrap();
    let name = p.file_name().unwrap().to_str().unwrap();
    if name == package_name {
        return p;
    }
    if p.join(package_name).is_dir() {
        return p.join(package_name);
    }
    if p.parent().unwrap().file_name().unwrap().to_str().unwrap() == package_name {
        return p.parent().unwrap().to_path_buf();
    }
    if let Some(parent) = p.parent() {
        if parent.join(package_name).is_dir() {
            return parent.join(package_name);
        }
        if let Some(parent) = parent.parent() {
            if parent.join(package_name).is_dir() {
                return parent.join(package_name);
            }
            if let Some(parent) = p.parent() {
                if parent.join(package_name).is_dir() {
                    return parent.join(package_name);
                }
            }
        }
    }
    info!("get_package_dir: {:?}", p);
    panic!("could not find package dir");
}

/// Migrate all databases for all collections.
pub async fn migrate_all() -> Result<()> {
    info!("migrate()");

    info!("session ok");
    migrate_common().await.context("migrate common")?;
    info!("common ok");
    use crate::models::common::collection::CollectionDbRow;
    use charybdis::operations::Find;

    let session = ScyllaDatabaseHandle::global_session()
        .await
        .context("scylla global session")?;
    let mut collections_stream: charybdis::stream::CharybdisModelStream<CollectionDbRow> =
        CollectionDbRow::find_all().execute(&session).await?;
    use futures::StreamExt;
    while let Some(Ok(c)) = collections_stream.next().await {
        if let Ok(c) = CollectionId::new(&c.collection_id) {
            migrate_collection(&c)
                .await
                .with_context(move || format!("migrate {:?}", c))?;
        }
    }

    info!("collections ok");

    Ok(())
}

/// Migrate the common database schema (the one not related to any collection).
pub async fn migrate_common() -> Result<()> {
    tracing::info!("migrate_common");
    with_redis_lock(
        "migrate_lock_common",
        async move { _migrate_common().await },
    )
    .await?
}

/// Migrate databases for a single collection.
pub async fn migrate_collection(c: &CollectionId) -> Result<()> {
    tracing::info!("migrate_collection {}", c.to_string());
    let c = c.clone();
    with_redis_lock(&format!("migrate_lock_2_{}", c), async move {
        _migrate_collection(c).await
    })
    .await?
}

/// Drop databases for a single collection.
pub async fn drop_collection(c: &CollectionId) -> Result<()> {
    tracing::info!("drop_collection {}", c.to_string());
    let c = c.clone();
    with_redis_lock(&format!("migrate_lock_2_{}", c), async move {
        _drop_collection(c).await
    })
    .await?
}

async fn _migrate_common() -> Result<()> {
    let session = ScyllaDatabaseHandle::global_session().await?;
    info!("initiate common migration");

    let common_path = get_package_dir()
        .join("src/models/common")
        .canonicalize()
        .unwrap();
    assert!(common_path.is_dir());
    let common_path = common_path.display().to_string();

    use charybdis::migrate::MigrationBuilder;
    let migration = MigrationBuilder::new()
        .keyspace(DEFAULT_KEYSPACE_NAME.to_string())
        .drop_and_replace(false)
        .current_dir(common_path)
        .build(session.get_session())
        .await;
    info!("create common migration OK");

    migration.run().await;
    info!("execute common migration OK");

    Ok(())
}

macro_rules! run_on_all_db_handles {
    ($id:tt) => {
        $id!(ScyllaDatabaseHandle);
        // $id!(ClickhouseDatabaseHandle);
        $id!(MeilisearchDatabaseHandle);
        // $id!(NebulaDatabaseHandle);
        $id!(S3DatabaseHandle);
        // $id!(SeekstormDatabaseHandle);
    };
}

async fn _migrate_collection(c: CollectionId) -> Result<()> {
    info!("migrate collection {}", c.to_string());
    let space = c.database_name()?;
    let c = c.clone();

    macro_rules! create_db {
        ($id:ident) => {
            // use crate::db_management::$id;
            $id::global_session()
                .await
                .context(format!("get global session for {}", stringify!($id)))?
                .create_space(&space)
                .await
                .context(format!("create space for {}", stringify!($id)))?;
        };
    }

    run_on_all_db_handles!(create_db);

    macro_rules! check_db {
        ($id:ident) => {
            if !$id::global_session()
                .await
                .context(format!("get global session for {}", stringify!($id)))?
                .space_exists(&space)
                .await
                .context(format!("check space exists for {}", stringify!($id)))?
            {
                anyhow::bail!(
                    "migrate error: space {space:?} missing on db {}",
                    stringify!($id)
                )
            }
        };
    }
    run_on_all_db_handles!(check_db);

    drop_redis_cache("query_nebula_schema", &c).await?;
    drop_redis_cache("query_scylla_schema", &c).await?;
    drop_redis_cache("query_meilisearch_schema", &c).await?;

    macro_rules! migrate_collection_space {
        ($id:ident) => {
            $id::migrate_collection_space(&c)
                .await
                .context(format!("migrate collection space for {}", stringify!($id)))?;
        };
    }
    run_on_all_db_handles!(migrate_collection_space);

    Ok(())
}

async fn _drop_collection(c: CollectionId) -> Result<()> {
    let c = c.clone();
    info!("dropping collection {}", c.to_string());
    let session = ScyllaDatabaseHandle::global_session().await?;

    use crate::models::common::collection::CollectionDbRow;
    // use charybdis::operations::Delete;
    CollectionDbRow::delete_by_collection_id(c.to_string())
        .execute(&session)
        .await
        .context("delete collection from scylla")?;

    let space = c.database_name()?;
    macro_rules! drop_db {
        ($id:ident) => {
            use crate::db_management::$id;
            $id::global_session()
                .await
                .context(format!("get global session for {}", stringify!($id)))?
                .drop_space(&space)
                .await?;
        };
    }

    run_on_all_db_handles!(drop_db);

    macro_rules! check_db {
        ($id:ident) => {
            if $id::global_session()
                .await
                .context(format!("get global session for {}", stringify!($id)))?
                .space_exists(&space)
                .await?
            {
                anyhow::bail!(
                    "drop collection error: space {space:?} still exists in {}",
                    stringify!($id)
                )
            }
        };
    }

    run_on_all_db_handles!(check_db);

    info!("collection {} dropped OK", c.to_string());

    Ok(())
}

#[tokio::test]
async fn test_create_drop_collection() {
    info!("test init");
    migrate_all().await.unwrap();
    let c = CollectionId::new("some_test_collection").unwrap();
    migrate_collection(&c).await.unwrap();
    migrate_collection(&c).await.unwrap();
    migrate_all().await.unwrap();
    migrate_collection(&c).await.unwrap();
    drop_collection(&c).await.unwrap();
    drop_collection(&c).await.unwrap();
}

/// API Client method to read the schema information from the database for a specific collection..
pub async fn get_collection_schema(c: CollectionId) -> Result<CollectionSchemaDynamic> {
    tracing::info!("get_collection_schema {}", c.to_string());
    Ok(CollectionSchemaDynamic {
        collection_id: c.clone(),
        scylla: crate::db_management::query_scylla_schema(&c).await?,
        nebula: crate::db_management::query_nebula_schema(&c).await?,
        meilisearch: crate::db_management::query_meilisearch_schema(&c).await?,
    })
}

/// Get extra charybdis codes that are not part of the collection migration.
/// Useful for adding tables that we don't implemented with the `#[model]` macro,
/// since they are part of that macro's implementation -- things like the graph tables.
pub fn get_extra_charybdis_codes() -> Vec<String> {
    vec![include_str!("../../src/models/collection/_scylla_graph_models.rs").to_string()]
}
