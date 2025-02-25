use std::sync::Arc;

use super::ScyllaDatabaseHandle;
use crate::{
    db_management::DatabaseSpaceManager, models::common::seekstorm_index::SeekstormIndexInfo,
};
use anyhow::Context;
use charybdis::operations::{Find, Insert};
use hoover3_types::{
    db_schema::DatabaseColumnType,
    identifier::{CollectionId, DatabaseIdentifier},
};
use seekstorm_client::{
    apis::{
        api_key_api::{create_apikey_api, delete_apikey_api},
        configuration::{ApiKey, Configuration},
        index_api::create_index_api,
    },
    models::{
        CreateApikeyApiRequest, CreateIndexApiRequest, FieldType, SchemaField, SimilarityType,
        TokenizerType,
    },
};

const MASTER_API_KEY: &str = "A6xnQhbz4Vx2HuGl4lXwZ5U2I8iziLRFnhP5eNfIRvQ="; // for master key 1234
const SERVICE_URL: &str = "http://localhost:80";
/// Seekstorm database handle.
pub struct SeekstormDatabaseHandle {}

pub struct SeekstormIndexHandle {
    _index_api_key: String,
    _index_id: i64,
}

async fn get_index_info_c(c: &CollectionId) -> Result<SeekstormIndexInfo, anyhow::Error> {
    get_index_info_d(&c.database_name()?).await
}

async fn get_index_info_d(d: &DatabaseIdentifier) -> Result<SeekstormIndexInfo, anyhow::Error> {
    let session = ScyllaDatabaseHandle::global_session().await?;
    let info = SeekstormIndexInfo::find_by_collection_id(d.to_string())
        .execute(&session)
        .await?;
    Ok(info)
}

impl DatabaseSpaceManager for SeekstormDatabaseHandle {
    type CollectionSessionType = SeekstormIndexHandle;

    async fn global_session() -> Result<std::sync::Arc<Self>, anyhow::Error> {
        Ok(Arc::new(SeekstormDatabaseHandle {}))
    }

    async fn collection_session(
        c: &CollectionId,
    ) -> Result<std::sync::Arc<Self::CollectionSessionType>, anyhow::Error> {
        let info = get_index_info_c(c).await?;
        if info.seekstorm_api_key.is_empty() || info.seekstorm_index_id == -1 {
            return Err(anyhow::anyhow!("Seekstorm not yet migrated"));
        }
        Ok(Arc::new(SeekstormIndexHandle {
            _index_api_key: info.seekstorm_api_key,
            _index_id: info.seekstorm_index_id,
        }))
    }

    async fn space_exists(&self, name: &DatabaseIdentifier) -> Result<bool, anyhow::Error> {
        if let Ok(info) = get_index_info_d(name).await {
            return Ok(info.seekstorm_api_key.len() > 0);
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
        let index_api_key = make_seekstorm_api_key()
            .await
            .context("make seekstorm api key")?;
        SeekstormIndexInfo::insert(&SeekstormIndexInfo {
            collection_id: name.to_string(),
            seekstorm_api_key: index_api_key,
            seekstorm_index_id: -1,
        })
        .execute(&session)
        .await?;
        Ok(())
    }

    async fn drop_space(&self, name: &DatabaseIdentifier) -> Result<(), anyhow::Error> {
        let session = ScyllaDatabaseHandle::global_session().await?;
        let Ok(info) = SeekstormIndexInfo::find_by_collection_id(name.to_string())
            .execute(&session)
            .await
        else {
            return Ok(());
        };
        drop_seekstorm_apikey(info.seekstorm_api_key)
            .await
            .context("drop seekstorm api key")?;
        SeekstormIndexInfo::delete_by_collection_id(name.to_string())
            .execute(&session)
            .await?;
        Ok(())
    }
    async fn migrate_collection_space(c: &CollectionId) -> Result<(), anyhow::Error> {
        let session = ScyllaDatabaseHandle::global_session().await?;
        let mut info = get_index_info_c(c).await?;
        if info.seekstorm_api_key.is_empty() {
            return Err(anyhow::anyhow!("Seekstorm api key missing from db"));
        }
        let mut config = Configuration::default();
        config.api_key = Some(ApiKey {
            prefix: None,
            key: MASTER_API_KEY.to_string(),
        });
        let scylla_schema = hoover3_types::db_schema::get_scylla_schema_from_inventory();
        let mut schema = vec![];

        for (table_name, table) in scylla_schema.tables {
            for column in table.columns {
                let column_name = format!("{}.{}", table_name, column.name);
                let field = SchemaField {
                    indexed: true,
                    stored: true,
                    field: column_name.to_string(),
                    field_type: match column._type {
                        DatabaseColumnType::String => FieldType::Text,
                        DatabaseColumnType::Int8 => FieldType::I8,
                        DatabaseColumnType::Int16 => FieldType::I16,
                        DatabaseColumnType::Int32 => FieldType::I32,
                        DatabaseColumnType::Int64 => FieldType::I64,
                        DatabaseColumnType::Float => FieldType::F32,
                        DatabaseColumnType::Double => FieldType::F64,
                        DatabaseColumnType::Boolean => FieldType::Bool,
                        DatabaseColumnType::Timestamp => FieldType::Timestamp,
                        _ => FieldType::Text,
                    },
                    facet: Some(true),
                    boost: None,
                };
                schema.push(field);
            }
        }
        config.base_path = SERVICE_URL.to_string();
        let r = create_index_api(
            &config,
            &info.seekstorm_api_key,
            CreateIndexApiRequest {
                index_name: c.database_name()?.to_string(),
                schema,
                similarity: Some(SimilarityType::Bm25fProximity),
                tokenizer: Some(TokenizerType::UnicodeAlphanumericFolded),
                synonyms: vec![],
            },
        )
        .await?;
        info.seekstorm_index_id = r;
        use charybdis::operations::Update;
        SeekstormIndexInfo::update(&info).execute(&session).await?;

        Ok(())
    }
}

async fn make_seekstorm_api_key() -> Result<String, anyhow::Error> {
    let mut config = Configuration::default();
    config.api_key = Some(ApiKey {
        prefix: None,
        key: MASTER_API_KEY.to_string(),
    });
    let apikey = create_apikey_api(
        &config,
        MASTER_API_KEY,
        CreateApikeyApiRequest {
            indices_max: i64::MAX,
            indices_size_max: i64::MAX,
            documents_max: i64::MAX,
            operations_max: i64::MAX,
            rate_limit: i64::MAX,
        },
    )
    .await?;
    Ok(apikey)
}

async fn drop_seekstorm_apikey(index_api_key: String) -> Result<(), anyhow::Error> {
    let mut config = Configuration::default();
    config.api_key = Some(ApiKey {
        prefix: None,
        key: MASTER_API_KEY.to_string(),
    });
    config.base_path = SERVICE_URL.to_string();
    delete_apikey_api(&config, MASTER_API_KEY, &index_api_key).await?;
    Ok(())
}
