//! Search API methods - for main search page
use std::collections::BTreeMap;
use std::time::Instant;

use hoover3_types::{
    db_schema::{
        DatabaseColumnType, DatabaseValue,
        DynamicQueryResult, DynamicQueryResponse, DatabaseServiceType,
    },
    identifier::CollectionId,
};
use meilisearch_sdk::search::Selectors;

use crate::db_management::{
    DatabaseSpaceManager, MeilisearchDatabaseHandle,
};

use super::database_explorer::{json_value_to_database_type, json_value_to_database_value};


/// Run a Meilisearch facet search query and return the results.
/// This function allows searching with faceting to get aggregated results by specific fields.
pub async fn search_facet_query(
    (collection_id, sql_query, facet_fields, hits_per_page): (CollectionId, String, Vec<String>, u64),
) -> anyhow::Result<DynamicQueryResponse> {
    let start_time = Instant::now();
    let session = MeilisearchDatabaseHandle::collection_session(&collection_id).await?;
    let result = session
        .search()
        .with_query(&sql_query)
        .with_facets(Selectors::Some(
            facet_fields.iter().map(|s| s.as_str()).collect::<Vec<_>>().as_slice()
        ))
        .with_hits_per_page(hits_per_page as usize)
        .execute::<serde_json::Value>()
        .await?;

    // Extract hits from the result
    let hits = result.hits;
    let facets = result.facet_distribution;

    let query_result = if hits.is_empty() {
        DynamicQueryResult {
            columns: vec![],
            rows: vec![],
            next_page: None,
        }
    } else {
        // Process hits similar to regular search
        let mut column_map = std::collections::BTreeMap::new();
        for hit in hits.iter() {
            let hit = &hit.result;
            if let serde_json::Value::Object(obj) = hit {
                for (k, _v) in obj.iter() {
                    if let Some(_vtype) = json_value_to_database_type(_v) {
                        if column_map.contains_key(k) {
                            if let Some(old_value) = column_map.get(k) {
                                if old_value != &_vtype {
                                    panic!("different types for column: {:?}", k);
                                }
                            }
                        }
                        column_map.insert(k.to_string(), _vtype);
                    }
                }
            }
        }

        // Add facet columns if facets are present
        if let Some(facet_dist) = &facets {
            for facet_name in facet_dist.keys() {
                column_map.insert(format!("facet_{}", facet_name), DatabaseColumnType::Object(BTreeMap::new()));
            }
        }

        let mut column_pos = std::collections::BTreeMap::new();
        for (i, col) in column_map.keys().enumerate() {
            column_pos.insert(col.clone(), i);
        }

        // Process rows including facet information
        let rows = hits
            .into_iter()
            .map(|r| match r.result {
                serde_json::Value::Object(o) => {
                    let mut pairs = o
                        .into_iter()
                        .map(|(_k, v)| (_k, json_value_to_database_value(v)))
                        .collect::<BTreeMap<_, _>>();

                    // Add facet information to each row if facets are present
                    if let Some(facet_dist) = &facets {
                        for (facet_name, facet_values) in facet_dist {
                            let facet_key = format!("facet_{}", facet_name);
                            pairs.insert(facet_key, Some(DatabaseValue::Object(
                                facet_values.iter()
                                    .map(|(value, count)| (value.to_string(), Some(DatabaseValue::Int64(*count as i64))))
                                    .collect()
                            )));
                        }
                    }

                    for column in column_map.keys() {
                        if !pairs.contains_key(column) {
                            pairs.insert(column.clone(), None);
                        }
                    }

                    let mut pairs = pairs.into_iter().collect::<Vec<_>>();
                    pairs.sort_by_key(|(_k, _v)| column_pos.get(&(_k.clone())).unwrap_or(&0));
                    pairs.into_iter().map(|(_k, v)| v).collect::<Vec<_>>()
                }
                _ => vec![],
            })
            .collect::<Vec<_>>();

        DynamicQueryResult {
            columns: column_map.into_iter().collect::<Vec<_>>(),
            rows,
            next_page: None,
        }
    };

    let elapsed = start_time.elapsed().as_secs_f64();
    let serialized_size = bincode::serialized_size(&query_result)?;

    Ok(DynamicQueryResponse {
        query: sql_query,
        db_type: DatabaseServiceType::Meilisearch,
        elapsed_seconds: elapsed,
        result_serialized_size_bytes: serialized_size as u64,
        result: Ok(query_result),
    })
}