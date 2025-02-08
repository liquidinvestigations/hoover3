use std::sync::Arc;

use crate::{db_management::DatabaseSpaceManager, models::common::seekstorm_index::SeekstormIndexInfo};
use anyhow::Context;
use hoover3_types::identifier::{CollectionId, DatabaseIdentifier};
use seekstorm_client::{apis::{api_key_api::{create_apikey_api,delete_apikey_api,get_apikey_indices_info_api}, configuration::{ApiKey, Configuration}, index_api::create_index_api}, models::{CreateApikeyApiRequest, CreateIndexApiRequest, SchemaField, SimilarityType, TokenizerType}};
use charybdis::operations::{Find, Insert};
use super::ScyllaDatabaseHandle;

const MASTER_API_KEY: &str = "A6xnQhbz4Vx2HuGl4lXwZ5U2I8iziLRFnhP5eNfIRvQ="; // for master key 1234
const SERVICE_URL: &str = "http://localhost:80";
pub struct SeekstormDatabaseHandle {

}

pub struct SeekstormIndexHandle {
    index_api_key: String,
    index_id: i32,

}

impl DatabaseSpaceManager for SeekstormDatabaseHandle {
    type CollectionSessionType = SeekstormIndexHandle;

    async fn global_session() -> Result<std::sync::Arc<Self>, anyhow::Error> {
        Ok(Arc::new(SeekstormDatabaseHandle {
        }))
    }

    async fn collection_session(
        c: &CollectionId,
    ) -> Result<std::sync::Arc<Self::CollectionSessionType>, anyhow::Error> {
        todo!()
    }

    async fn space_exists(&self, name: &DatabaseIdentifier) -> Result<bool, anyhow::Error> {
        let session = ScyllaDatabaseHandle::global_session().await?;
        if let Ok(_x) = SeekstormIndexInfo::find_by_collection_id(name.to_string()).execute(&session).await {
            return Ok(_x.seekstorm_api_key.len() > 0);
        }
        Ok(false)
    }

    async fn list_spaces(&self) -> Result<Vec<DatabaseIdentifier>, anyhow::Error> {
        let session = ScyllaDatabaseHandle::global_session().await?;
        let mut result = Vec::new();
        let mut stream = SeekstormIndexInfo::find_all().execute(&session).await?;
        use futures::StreamExt;
        while let Some(Ok(c)) = stream.next().await {
            result.push(DatabaseIdentifier::new(&c.collection_id)?);
        }
        Ok(result)
    }

    async fn create_space(&self, name: &DatabaseIdentifier) -> Result<(), anyhow::Error> {
        if self.space_exists(name).await? {
            return Ok(());
        }
        let session = ScyllaDatabaseHandle::global_session().await?;
        let index_api_key = make_seekstorm_api_key().await.context("make seekstorm api key")?;
        SeekstormIndexInfo::insert(&SeekstormIndexInfo {
            collection_id: name.to_string(),
            seekstorm_api_key: index_api_key,
            seekstorm_index_id: -1,
        }).execute(&session).await?;
        Ok(())
    }

    async fn drop_space(&self, name: &DatabaseIdentifier) -> Result<(), anyhow::Error> {
        let session = ScyllaDatabaseHandle::global_session().await?;
        let Ok(info) = SeekstormIndexInfo::find_by_collection_id(name.to_string()).execute(&session).await else {
            return Ok(());
        };
        drop_seekstorm_apikey(info.seekstorm_api_key).await.context("drop seekstorm api key")?;
        SeekstormIndexInfo::delete_by_collection_id(name.to_string()).execute(&session).await?;
        Ok(())
    }

}



async fn make_seekstorm_api_key() -> Result<String, anyhow::Error> {
    let mut config = Configuration::default();
    config.api_key = Some(ApiKey {
        prefix: None,
        key: MASTER_API_KEY.to_string(),
    });
    let apikey = create_apikey_api(&config, MASTER_API_KEY, CreateApikeyApiRequest {
        indices_max: i64::MAX,
        indices_size_max: i64::MAX,
        documents_max: i64::MAX,
        operations_max: i64::MAX,
        rate_limit: i64::MAX,
    }).await?;
    Ok(apikey)
}

async fn drop_seekstorm_apikey(index_api_key: String) -> Result<(), anyhow::Error> {
    let mut config = Configuration::default();
    config.api_key = Some(ApiKey {
        prefix: None,
        key: MASTER_API_KEY.to_string(),
    });
    delete_apikey_api(&config, MASTER_API_KEY, &index_api_key).await?;
    Ok(())
}
