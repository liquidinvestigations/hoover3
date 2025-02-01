use std::{env, sync::Arc, time::Duration};

use meilisearch_sdk::client::*;
use meilisearch_sdk::task_info::TaskInfo;
use tokio::sync::OnceCell;
use tokio::sync::RwLock;
use std::collections::HashMap;
use super::{CollectionId, DatabaseIdentifier, DatabaseSpaceManager};

pub type MeilisearchDatabaseHandle = Client;

fn new_client() -> Client {
    let url = env::var("MEILI_URL").unwrap_or_else(|_| "http://localhost:7700".to_owned());
    let key = env::var("MEILI_MASTER_KEY").unwrap_or_else(|_| "1234".to_owned());
    Client::new(url, Some(key)).expect("cannot build client")
}

pub async fn meilisearch_wait_for_task(task: TaskInfo) -> anyhow::Result<()> {
    let res = task.wait_for_completion(&MeilisearchDatabaseHandle::global_session().await?.clone(), None, None).await?;
    if let  meilisearch_sdk::tasks::Task::Succeeded { .. } = res {
        Ok(())
    } else {
        anyhow::bail!("meilisearch task error: {:?}", res);
    }
}

impl DatabaseSpaceManager for MeilisearchDatabaseHandle {
    type CollectionSessionType = meilisearch_sdk::indexes::Index;
    async fn global_session() -> anyhow::Result<Arc<Self>> {
        static MEILISEARCH_CLIENT: OnceCell<Arc<Client>> = OnceCell::const_new();
        Ok(MEILISEARCH_CLIENT
            .get_or_init(|| async { Arc::new(new_client()) })
            .await
            .clone())
    }
    async fn collection_session(_c: &CollectionId) -> Result<Arc<Self::CollectionSessionType>, anyhow::Error> {

            static HASH: OnceCell<RwLock<HashMap<CollectionId, Arc<meilisearch_sdk::indexes::Index>>>> =
                OnceCell::const_new();
            let h = HASH
                .get_or_init(|| async move { RwLock::new(HashMap::new()) })
                .await;
            // try to fetch from hashmap
            {
                let h = h.read().await;
                if let Some(s) = h.get(_c) {
                    return Ok(s.clone());
                }
            }
            // if not found, open new session
            let s = {
                let mut h = h.write().await;
                let s = {
                    let client = Self::global_session().await?;
                    let index = client.get_index(&_c.database_name()?.to_string()).await?;
                    Arc::new(index)
                };
                h.insert(_c.clone(), s.clone());
                s
            };
            Ok(s)
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
            meilisearch_sdk::tasks::Task::Succeeded { content: _ } => Ok(()),
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
            meilisearch_sdk::tasks::Task::Succeeded { content: _ } => Ok(()),
            _x => {
                anyhow::bail!(
                    "drop index task not finished successfully after waiting! {:?}",
                    _x
                )
            }
        }
    }
}
