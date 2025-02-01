pub use hoover3_types::identifier::{CollectionId, DatabaseIdentifier};
pub(crate) mod redis;

mod clickhouse;
pub use clickhouse::ClickhouseDatabaseHandle;

mod meilisearch;
pub use meilisearch::MeilisearchDatabaseHandle;
pub use meilisearch::meilisearch_wait_for_task;

mod nebula;
pub use nebula::NebulaDatabaseHandle;
pub use nebula::NebulaDatabaseHandleExt;

mod scylla;
pub use scylla::ScyllaDatabaseHandle;

mod seaweed;
pub use seaweed::S3DatabaseHandle;

use std::sync::Arc;

#[allow(async_fn_in_trait)]
pub trait DatabaseSpaceManager {
    type CollectionSessionType;
    async fn global_session() -> Result<Arc<Self>, anyhow::Error>;
    async fn collection_session(c: &CollectionId) -> Result<Arc<Self::CollectionSessionType>, anyhow::Error>;

    async fn space_exists(&self, name: &DatabaseIdentifier) -> Result<bool, anyhow::Error>;
    async fn list_spaces(&self) -> Result<Vec<DatabaseIdentifier>, anyhow::Error>;
    async fn create_space(&self, name: &DatabaseIdentifier) -> Result<(), anyhow::Error>;
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
