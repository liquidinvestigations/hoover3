pub mod nebula;
pub use nebula::{_migrate_nebula_collection, nebula_get_tags_schema};

pub mod schema;

use anyhow::Context;
use anyhow::Result;
use std::path::PathBuf;
use tracing::info;

use crate::db_management::CollectionId;
use crate::{db_management::DatabaseSpaceManager, db_management::ScyllaDatabaseHandle};
use hoover3_types::identifier::DEFAULT_KEYSPACE_NAME;

use super::db_management::redis::with_redis_lock;

/// sometimes we run from workspace root. Sometimes we run from package root.
/// We need to point to correct migration dirs; so we need to identify this package's dir.
/// This function tries a few variations and finds the `hoover3_database` package root.
pub(crate) fn get_package_dir() -> PathBuf {
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

pub async fn migrate_common() -> Result<()> {
    with_redis_lock(
        "migrate_lock_common",
        async move { _migrate_common().await },
    )
    .await?
}

pub async fn migrate_collection(c: &CollectionId) -> Result<()> {
    let c = c.clone();
    with_redis_lock(&format!("migrate_lock_2_{}", c), async move {
        _migrate_collection(c).await
    })
    .await?
}

pub async fn drop_collection(c: &CollectionId) -> Result<()> {
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

async fn _migrate_collection(c: CollectionId) -> Result<()> {
    info!("migrate collection {}", c.to_string());
    let space = c.database_name()?;
    let c = c.clone();
    scylla_migrate_collection(&c).await?;

    macro_rules! create_db {
        ($id:ident) => {
            use crate::db_management::$id;
            $id::global_session()
                .await
                .context(format!("get global session for {}", stringify!($id)))?
                .create_space(&space)
                .await
                .context(format!("create space for {}", stringify!($id)))?;
        };
    }

    create_db!(ClickhouseDatabaseHandle);
    create_db!(MeilisearchDatabaseHandle);
    create_db!(NebulaDatabaseHandle);
    create_db!(ScyllaDatabaseHandle);
    create_db!(S3DatabaseHandle);

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

    check_db!(ClickhouseDatabaseHandle);
    check_db!(MeilisearchDatabaseHandle);
    check_db!(NebulaDatabaseHandle);
    check_db!(ScyllaDatabaseHandle);
    check_db!(S3DatabaseHandle);

    _migrate_nebula_collection(&c).await?;

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

    drop_db!(ClickhouseDatabaseHandle);
    drop_db!(MeilisearchDatabaseHandle);
    drop_db!(NebulaDatabaseHandle);
    drop_db!(ScyllaDatabaseHandle);
    drop_db!(S3DatabaseHandle);

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

    check_db!(ClickhouseDatabaseHandle);
    check_db!(MeilisearchDatabaseHandle);
    check_db!(NebulaDatabaseHandle);
    check_db!(ScyllaDatabaseHandle);
    check_db!(S3DatabaseHandle);

    info!("collection {} dropped OK", c.to_string());

    Ok(())
}

async fn scylla_migrate_collection(c: &CollectionId) -> Result<()> {
    info!("scylla_migrate_collection {}", c.to_string());
    let session = ScyllaDatabaseHandle::open_session(c.database_name()?).await?;
    let space_name = c.database_name()?.to_string();

    info!("initiate collection {space_name} migration");
    let collection_model_path = get_package_dir()
        .join("src/models/collection")
        .canonicalize()
        .unwrap();
    assert!(collection_model_path.is_dir());
    let collection_model_path = collection_model_path.display().to_string();

    use charybdis::migrate::MigrationBuilder;
    let migration = MigrationBuilder::new()
        .keyspace(space_name.clone())
        .drop_and_replace(false)
        .current_dir(collection_model_path)
        .build(session.get_session())
        .await;
    info!("create collection {space_name} migration OK");

    migration.run().await;
    info!("execute collection {space_name} migration OK");

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
