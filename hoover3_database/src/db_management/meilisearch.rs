use std::{env, sync::Arc, time::Duration};

use meilisearch_sdk::client::*;
use tokio::sync::OnceCell;

use super::{CollectionId, DatabaseIdentifier, DatabaseSpaceManager};

pub type MeilisearchDatabaseHandle = Client;

fn new_client() -> Client {
    let url = env::var("MEILI_URL").unwrap_or_else(|_| "http://localhost:7700".to_owned());
    let key = env::var("MEILI_MASTER_KEY").unwrap_or_else(|_| "1234".to_owned());
    Client::new(url, Some(key)).expect("cannot build client")
}

impl DatabaseSpaceManager for MeilisearchDatabaseHandle {
    async fn global_session() -> anyhow::Result<Arc<Self>> {
        static MEILISEARCH_CLIENT: OnceCell<Arc<Client>> = OnceCell::const_new();
        Ok(MEILISEARCH_CLIENT
            .get_or_init(|| async { Arc::new(new_client()) })
            .await
            .clone())
    }
    async fn collection_session(_c: &CollectionId) -> Result<Arc<Self>, anyhow::Error> {
        // meilisearch does not have a client-per-database config
        Self::global_session().await
    }
    async fn space_exists(&self, name: &DatabaseIdentifier) -> anyhow::Result<bool> {
        let name = name.to_string();

        let index = self.get_index(&name).await;
        if let Ok(index) = index {
            if index.uid.eq(&name) {
                return Ok(true);
            }
        }
        Ok(false)
    }
    async fn list_spaces(&self) -> anyhow::Result<Vec<DatabaseIdentifier>> {
        let indexes = self.get_indexes().await?;
        Ok(indexes
            .results
            .iter()
            .filter_map(|i| DatabaseIdentifier::new(i.uid.clone()).ok())
            .collect::<Vec<_>>())
    }
    async fn create_space(&self, name: &DatabaseIdentifier) -> Result<(), anyhow::Error> {
        if self.space_exists(name).await? {
            return Ok(());
        }
        let task = self.create_index(&name.to_string(), Some("id")).await?;
        let res = self
            .wait_for_task(
                &task,
                Some(Duration::from_millis(500)),
                Some(Duration::from_millis(50000)),
            )
            .await;
        match res? {
            meilisearch_sdk::tasks::Task::Succeeded { content: _ } => return Ok(()),
            _x => {
                anyhow::bail!(
                    "create index task not finished successfully after waiting! {:?}",
                    _x
                )
            }
        }
    }
    async fn drop_space(&self, name: &DatabaseIdentifier) -> Result<(), anyhow::Error> {
        if !self.space_exists(name).await? {
            return Ok(());
        }
        let task = self.delete_index(&name.to_string()).await?;
        let res = self
            .wait_for_task(
                &task,
                Some(Duration::from_millis(500)),
                Some(Duration::from_millis(50000)),
            )
            .await;
        match res? {
            meilisearch_sdk::tasks::Task::Succeeded { content: _ } => return Ok(()),
            _x => {
                anyhow::bail!(
                    "drop index task not finished successfully after waiting! {:?}",
                    _x
                )
            }
        }
    }
}
