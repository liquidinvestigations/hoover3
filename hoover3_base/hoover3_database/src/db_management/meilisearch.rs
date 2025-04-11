//! Meilisearch database management module that handles index operations and search functionality.
//! Implements the DatabaseSpaceManager trait for managing Meilisearch indexes and provides
//! utilities for task management and waiting.
use std::{env, sync::Arc, time::Duration};

use super::{CollectionId, DatabaseIdentifier, DatabaseSpaceManager};
use hoover3_types::db_schema::DatabaseColumnType;
use meilisearch_sdk::client::*;
use meilisearch_sdk::task_info::TaskInfo;
use std::collections::HashMap;
use tokio::sync::OnceCell;
use tokio::sync::RwLock;
use tracing::info;

use hoover3_types::db_schema::MeilisearchDatabaseSchema;

use crate::db_management::redis::with_redis_cache;
use crate::models::collection::get_scylla_schema_from_inventory;

/// Meilisearch database handle type alias.
pub type MeilisearchDatabaseHandle = Client;

fn new_client() -> Client {
    let url = env::var("MEILI_URL").unwrap_or_else(|_| "http://localhost:7700".to_owned());
    let key = env::var("MEILI_MASTER_KEY").unwrap_or_else(|_| "1234".to_owned());
    Client::new(url, Some(key)).expect("cannot build client")
}

/// Wait for a Meilisearch task to complete and return the result.
pub async fn meilisearch_wait_for_task(task: TaskInfo) -> anyhow::Result<()> {
    let res = task
        .wait_for_completion(
            &MeilisearchDatabaseHandle::global_session().await?.clone(),
            None,
            None,
        )
        .await?;
    if let meilisearch_sdk::tasks::Task::Succeeded { .. } = res {
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
    async fn collection_session(
        _c: &CollectionId,
    ) -> Result<Arc<Self::CollectionSessionType>, anyhow::Error> {
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
            meilisearch_sdk::tasks::Task::Succeeded { content: _ } => {}
            _x => {
                anyhow::bail!(
                    "create index task not finished successfully after waiting! {:?}",
                    _x
                )
            }
        }
        tracing::info!("create index task finished successfully");

        Ok(())
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
    async fn migrate_collection_space(_c: &CollectionId) -> Result<(), anyhow::Error> {
        info!("meilisearch: migrate collection space {}", _c.to_string());
        let new_settings = get_index_settings()?;
        let client = Self::global_session().await?;

        // don't use the collection session for this - that assumes all migraitons were applied
        // let index = Self::collection_session(_c).await?;
        let index = client.get_index(&_c.database_name()?.to_string()).await?;

        let old_settings = index.get_settings().await?;
        if serde_json::to_value(&old_settings)? == serde_json::to_value(&new_settings)? {
            info!(
                "no change in collection index {} settings, skipping",
                _c.to_string()
            );
            return Ok(());
        }

        info!(
            "setting meilisearch index settings for collection {}: {:#?}",
            _c.to_string(),
            &new_settings
        );
        let _task = index.set_settings(&new_settings).await?;
        info!("waiting for meilisearch change setting task...");
        let _task = client
            .wait_for_task(
                &_task,
                Some(Duration::from_millis(500)),
                Some(Duration::from_millis(50000)),
            )
            .await?;
        Ok(())
    }
}

/// API Client method to get the Meilisearch database schema for a collection.
pub async fn query_meilisearch_schema(
    c: &CollectionId,
) -> anyhow::Result<MeilisearchDatabaseSchema> {
    let c = c.clone();
    with_redis_cache("query_meilisearch_schema", 60, _get_meilisearch_schema, &c).await
}

async fn _get_meilisearch_schema(c: CollectionId) -> anyhow::Result<MeilisearchDatabaseSchema> {
    tracing::info!("get_meilisearch_schema {}", c.to_string());
    let client = MeilisearchDatabaseHandle::collection_session(&c).await?;
    let stats = client.get_stats().await?;

    Ok(MeilisearchDatabaseSchema {
        doc_count: stats.number_of_documents as i64,
        fields: stats
            .field_distribution
            .into_iter()
            .map(|(k, v)| (k, v as i64))
            .collect(),
    })
}

pub struct SearchFieldConfigurations {
    pub filterable_attributes: Vec<String>,
    pub sortable_attributes: Vec<String>,
}

pub fn get_field_configurations() -> SearchFieldConfigurations {
    let mut filterable_attributes = vec![];
    let mut sortable_attributes = vec![];

    let schema = get_scylla_schema_from_inventory();

    for table in schema.tables.values() {
        for column in table.columns.iter() {
            let field_name = format!("{}:{}", table.name.to_string(), column.name.to_string());
            let Some(field_definition) = column.field_definition.as_ref() else {
                info!("skip: {} - no field definition", field_name);
                continue;
            };
            if !matches!(
                field_definition.field_type,
                DatabaseColumnType::String
                    | DatabaseColumnType::Int8
                    | DatabaseColumnType::Int16
                    | DatabaseColumnType::Int32
                    | DatabaseColumnType::Int64
                    | DatabaseColumnType::Float
                    | DatabaseColumnType::Double
                    | DatabaseColumnType::Boolean
                    | DatabaseColumnType::Timestamp
            ) {
                info!(
                    "skip: {} - field type {:?} not supported",
                    field_name, field_definition.field_type
                );
                continue;
            }
            if !field_definition.search_index {
                continue;
            }
            if field_definition.search_facet {
                filterable_attributes.push(field_name.clone());
            }
            if field_definition.search_index {
                sortable_attributes.push(field_name);
            }
        }
    }
    SearchFieldConfigurations {
        filterable_attributes,
        sortable_attributes,
    }
}

fn get_index_settings() -> anyhow::Result<meilisearch_sdk::settings::Settings> {
    let field_configs = get_field_configurations();

    let settings = meilisearch_sdk::settings::Settings {
        synonyms: None,
        stop_words: None,
        ranking_rules: None,
        filterable_attributes: Some(field_configs.filterable_attributes),
        sortable_attributes: Some(field_configs.sortable_attributes),
        distinct_attribute: None,
        searchable_attributes: None, // Some(field_configs.searchable_fields),
        displayed_attributes: None,
        pagination: None,
        faceting: None,
        typo_tolerance: None,
        dictionary: None,
        proximity_precision: None,
        search_cutoff_ms: None,
        separator_tokens: None,
        non_separator_tokens: None,
    };

    Ok(settings)
}

/// Check if a table should be indexed in Meilisearch.
pub fn search_index_include_table(table_name: &str) -> anyhow::Result<bool> {
    let table_name = DatabaseIdentifier::new(table_name)?;
    let schema = get_scylla_schema_from_inventory();
    let Some(table) = schema.tables.get(&table_name) else {
        return Ok(true);
    };
    let should_index_table = table.columns.iter().any(|c| {
        let Some(def) = &c.field_definition else {
            return false;
        };
        def.search_store
    });
    Ok(should_index_table)
}
