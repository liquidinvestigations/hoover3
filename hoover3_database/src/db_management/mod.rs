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
pub use meilisearch::MeilisearchDatabaseHandle;

mod nebula;
pub use nebula::nebula_execute_retry;
pub use nebula::NebulaDatabaseHandle;

mod scylla;
pub use scylla::ScyllaDatabaseHandle;

mod seaweed;
pub use seaweed::S3DatabaseHandle;

use std::sync::Arc;

/// Trait defining the interface for managing database spaces and sessions.
/// To be implemented for each database backend.
#[allow(async_fn_in_trait)]
pub trait DatabaseSpaceManager {
    /// Type representing a collection-specific database session
    type CollectionSessionType;

    /// Creates a new global database session
    async fn global_session() -> Result<Arc<Self>, anyhow::Error>;

    /// Creates a new database session for a specific collection
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
}

async fn _test_db_session<T: DatabaseSpaceManager>() {
    let test_db_name = "test_1_xxxyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy"; // len = 48
    let test_db_name = DatabaseIdentifier::new(test_db_name).unwrap();
    let s = T::global_session().await.unwrap();

    for _i in 0..3 {
        s.create_space(&test_db_name).await.unwrap();
        assert!(s.list_spaces().await.unwrap().contains(&test_db_name));
        assert!(s.space_exists(&test_db_name).await.unwrap());
    }

    for _i in 0..3 {
        s.drop_space(&test_db_name).await.unwrap();
        assert!(!s.space_exists(&test_db_name).await.unwrap());
        assert!(!s.list_spaces().await.unwrap().contains(&test_db_name));
    }

    for _i in 0..3 {
        s.create_space(&test_db_name).await.unwrap();
        assert!(s.list_spaces().await.unwrap().contains(&test_db_name));
        assert!(s.space_exists(&test_db_name).await.unwrap());
        s.drop_space(&test_db_name).await.unwrap();
        assert!(!s.space_exists(&test_db_name).await.unwrap());
        assert!(!s.list_spaces().await.unwrap().contains(&test_db_name));
    }
}

#[tokio::test]
async fn test_db_sessions_seaweed() {
    use seaweed::S3DatabaseHandle;
    _test_db_session::<S3DatabaseHandle>().await;
}

#[tokio::test]
async fn test_db_sessions_clickhouse() {
    use clickhouse::ClickhouseDatabaseHandle;
    _test_db_session::<ClickhouseDatabaseHandle>().await;
}

#[tokio::test]
async fn test_db_sessions_meilisearch() {
    use meilisearch::MeilisearchDatabaseHandle;
    _test_db_session::<MeilisearchDatabaseHandle>().await;
}

#[tokio::test]
async fn test_db_sessions_nebula() {
    use nebula::NebulaDatabaseHandle;
    _test_db_session::<NebulaDatabaseHandle>().await;
}

#[tokio::test]
async fn test_db_sessions_scylla() {
    _test_db_session::<scylla::ScyllaDatabaseHandle>().await;
}
