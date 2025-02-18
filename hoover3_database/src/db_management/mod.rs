//! Core database management module that defines common interfaces and traits for different
//! database backends. Provides the DatabaseSpaceManager trait and re-exports specific
//! database implementations.
pub use hoover3_types::identifier::{CollectionId, DatabaseIdentifier};
pub(crate) mod redis;
pub use redis::{with_redis_cache, with_redis_lock};

mod clickhouse;
pub use clickhouse::ClickhouseDatabaseHandle;

mod meilisearch;
pub use meilisearch::meilisearch_wait_for_task;
pub use meilisearch::query_meilisearch_schema;
pub use meilisearch::MeilisearchDatabaseHandle;

mod nebula;
mod nebula_migrate;
pub use nebula::nebula_execute_retry;
pub use nebula::NebulaDatabaseHandle;
pub use nebula_migrate::query_nebula_schema;

mod scylla;
mod scylla_migrate;
pub use scylla::ScyllaDatabaseHandle;
pub use scylla_migrate::query_scylla_schema;

mod seaweed;
pub use seaweed::S3DatabaseHandle;

mod seekstorm;
pub use seekstorm::SeekstormDatabaseHandle;

use std::sync::Arc;


/// Trait defining the interface for managing database spaces and sessions.
/// To be implemented for each database backend.
#[allow(async_fn_in_trait)]
pub trait DatabaseSpaceManager {
    /// Type representing a collection-specific database session
    type CollectionSessionType;

    /// Creates a new global database session
    async fn global_session() -> Result<Arc<Self>, anyhow::Error>;

    /// Creates a new database session for a specific collection. Must have all migrations applied.
    async fn collection_session(
        c: &CollectionId,
    ) -> Result<Arc<Self::CollectionSessionType>, anyhow::Error>;

    /// Checks if a database space exists by name
    async fn space_exists(&self, name: &DatabaseIdentifier) -> Result<bool, anyhow::Error>;

    /// Returns a list of all database spaces
    async fn list_spaces(&self) -> Result<Vec<DatabaseIdentifier>, anyhow::Error>;

    /// Creates a new database space with the given name
    async fn create_space(&self, name: &DatabaseIdentifier) -> Result<(), anyhow::Error>;

    /// Drops/deletes an existing database space
    async fn drop_space(&self, name: &DatabaseIdentifier) -> Result<(), anyhow::Error>;

    /// Migrate database space to compiled schema
    async fn migrate_collection_space(c: &CollectionId) -> Result<(), anyhow::Error>;
}


async fn _test_db_session<T: DatabaseSpaceManager>() -> Result<(), anyhow::Error> {
    let name = format!(
        "test_db_session_{}",
        std::any::type_name::<T>()
            .split("::")
            .last()
            .unwrap()
            .to_lowercase()
            .replace(">", "")
            .as_str().split_at(6).0
    );
    let test_db_name = DatabaseIdentifier::new(&name)?;
    let s = T::global_session().await?;

    for _i in 0..3 {
        s.create_space(&test_db_name).await?;
        assert!(s.list_spaces().await?.contains(&test_db_name));
        assert!(s.space_exists(&test_db_name).await?);
    }

    for _i in 0..3 {
        s.drop_space(&test_db_name).await?;
        assert!(!s.space_exists(&test_db_name).await?);
        assert!(!s.list_spaces().await?.contains(&test_db_name));
    }

    for _i in 0..3 {
        s.create_space(&test_db_name).await?;
        assert!(s.list_spaces().await?.contains(&test_db_name));
        assert!(s.space_exists(&test_db_name).await?);
        s.drop_space(&test_db_name).await?;
        assert!(!s.space_exists(&test_db_name).await?);
        assert!(!s.list_spaces().await?.contains(&test_db_name));
    }

    let c = CollectionId::new(&name)?;
    s.create_space(&c.database_name()?).await?;
    T::migrate_collection_space(&c).await?;
    let c_s = T::collection_session(&c).await?;
    drop(c_s);
    s.drop_space(&c.database_name()?).await?;

    Ok(())
}

#[tokio::test]
async fn test_db_sessions() -> Result<(), anyhow::Error> {
    use crate::migrate::migrate_common;
    migrate_common().await?;

    _test_db_session::<scylla::ScyllaDatabaseHandle>().await?;

    use seaweed::S3DatabaseHandle;
    _test_db_session::<S3DatabaseHandle>().await?;

    use clickhouse::ClickhouseDatabaseHandle;
    _test_db_session::<ClickhouseDatabaseHandle>().await?;

    use meilisearch::MeilisearchDatabaseHandle;
    _test_db_session::<MeilisearchDatabaseHandle>().await?;

    use nebula::NebulaDatabaseHandle;
    _test_db_session::<NebulaDatabaseHandle>().await?;

    _test_db_session::<SeekstormDatabaseHandle>().await?;

    Ok(())
}
