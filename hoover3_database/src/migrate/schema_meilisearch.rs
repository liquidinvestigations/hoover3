use hoover3_types::{db_schema::MeilisearchDatabaseSchema, identifier::CollectionId};

use crate::db_management::{with_redis_cache, DatabaseSpaceManager, MeilisearchDatabaseHandle};

pub async fn get_meilisearch_schema(c: &CollectionId) -> anyhow::Result<MeilisearchDatabaseSchema> {
    let c = c.clone();
    with_redis_cache("meilisearch_get_schema", 60, _get_meilisearch_schema, &c).await
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
