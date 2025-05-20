//! Search API methods - for main search page
use std::collections::BTreeMap;
use std::time::Instant;

use hoover3_types::{
    db_schema::{
        DatabaseColumnType, DatabaseServiceType, DatabaseValue, DynamicQueryResponse,
        DynamicQueryResult,
    },
    identifier::CollectionId,
};
use meilisearch_sdk::search::Selectors;

use crate::db_management::{DatabaseSpaceManager, MeilisearchDatabaseHandle};

use super::database_explorer::{json_value_to_database_type, json_value_to_database_value};

/// Run a Meilisearch facet search query and return the results.
/// This function allows searching with faceting to get aggregated results by specific fields.
pub async fn search_facet_query(
    (collection_id, search_q, facet_fields, hits_per_page): (
        CollectionId,
        String,
        Vec<String>,
        u64,
    ),
) -> anyhow::Result<DynamicQueryResponse> {
    let start_time = Instant::now();
    let session = MeilisearchDatabaseHandle::collection_session(&collection_id).await?;
    let result = session
        .search()
        .with_query(&search_q)
        .with_facets(Selectors::Some(
            facet_fields
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .as_slice(),
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
                column_map.insert(
                    format!("facet_{}", facet_name),
                    DatabaseColumnType::Object(BTreeMap::new()),
                );
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
                            pairs.insert(
                                facet_key,
                                Some(DatabaseValue::Object(
                                    facet_values
                                        .iter()
                                        .map(|(value, count)| {
                                            (
                                                value.to_string(),
                                                Some(DatabaseValue::Int64(*count as i64)),
                                            )
                                        })
                                        .collect(),
                                )),
                            );
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
        query: search_q,
        db_type: DatabaseServiceType::Meilisearch,
        elapsed_seconds: elapsed,
        result_serialized_size_bytes: serialized_size as u64,
        result: Ok(query_result),
    })
}

/// Run a Meilisearch search query with highlighting enabled and return the results.
/// This function allows searching with highlighting to emphasize matching terms in the results.
pub async fn search_highlight_query(
    (collection_id, search_q, hits_per_page): (CollectionId, String, u64),
) -> anyhow::Result<DynamicQueryResponse> {
    let start_time = Instant::now();
    let session = MeilisearchDatabaseHandle::collection_session(&collection_id).await?;
    let result = session
        .search()
        .with_query(&search_q)
        .with_attributes_to_highlight(Selectors::Some(&["*"]))
        .with_highlight_pre_tag("<b class=search-result-highlight-span>")
        .with_highlight_post_tag("</b>")
        .with_hits_per_page(hits_per_page as usize)
        .execute::<serde_json::Value>()
        .await?;

    // Extract hits and highlights from the result
    let highlights: Vec<_> = result
        .hits
        .iter()
        .map(|hit| hit.formatted_result.clone())
        .collect();
    let hits = result.hits;

    let query_result = if hits.is_empty() {
        DynamicQueryResult {
            columns: vec![],
            rows: vec![],
            next_page: None,
        }
    } else {
        // Process hits to get column types
        let mut column_map = std::collections::BTreeMap::new();
        for (hit, highlight) in hits.iter().zip(highlights.iter()) {
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

        let mut column_pos = std::collections::BTreeMap::new();
        for (i, col) in column_map.keys().enumerate() {
            column_pos.insert(col.clone(), i);
        }

        // Process rows including both original and highlighted data
        let rows = hits
            .into_iter()
            .zip(highlights.into_iter())
            .map(|(r, highlight)| {
                let mut original_pairs = BTreeMap::new();
                let mut highlight_pairs = BTreeMap::new();

                // Process original data
                if let serde_json::Value::Object(o) = r.result {
                    for (k, v) in o {
                        original_pairs.insert(k, json_value_to_database_value(v));
                    }
                }

                // Process highlight data
                if let Some(highlight_obj) = highlight {
                    for (k, v) in highlight_obj {
                        // Only include fields that have highlighting markup
                        if v.to_string()
                            .contains("<b class=search-result-highlight-span>")
                        {
                            let v = v.to_string();
                            let v = trim_response_value(&v);
                            highlight_pairs.insert(k, Some(DatabaseValue::String(v.to_string())));
                        }
                    }
                }

                // Combine both into a single row, prioritizing highlighted fields
                let mut combined_pairs = BTreeMap::new();
                for column in column_map.keys() {
                    if let Some(value) = highlight_pairs.get(column) {
                        combined_pairs.insert(column.clone(), value.clone());
                    } else if let Some(value) = original_pairs.get(column) {
                        combined_pairs.insert(column.clone(), value.clone());
                    } else {
                        combined_pairs.insert(column.clone(), None);
                    }
                }

                let mut pairs = combined_pairs.into_iter().collect::<Vec<_>>();
                pairs.sort_by_key(|(_k, _v)| column_pos.get(&(_k.clone())).unwrap_or(&0));
                pairs.into_iter().map(|(_k, v)| v).collect::<Vec<_>>()
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
        query: search_q,
        db_type: DatabaseServiceType::Meilisearch,
        elapsed_seconds: elapsed,
        result_serialized_size_bytes: serialized_size as u64,
        result: Ok(query_result),
    })
}

fn trim_response_value(value: &str) -> String {
    // If no highlight markup, return as-is
    if !value.contains("<b class=search-result-highlight-span>") {
        return value.to_string();
    }

    let mut result = String::new();
    let mut last_end = 0;

    // Find each highlight and add context around it
    for (start, _) in value.match_indices("<b class=search-result-highlight-span>") {
        // Add ellipsis between highlights
        if start > last_end {
            result.push_str(" ... ");
        }

        // Add text before highlight
        let context_start = if start >= 20 {
            // Find the character boundary 20 chars before
            let mut chars = value[..start].chars().rev().take(20);
            let mut count = 0;
            let mut pos = start;
            while let Some(_) = chars.next() {
                pos = value[..pos]
                    .char_indices()
                    .next_back()
                    .map(|(i, _)| i)
                    .unwrap_or(0);
                count += 1;
                if count >= 20 {
                    break;
                }
            }
            pos
        } else {
            0
        };

        if context_start < start {
            result.push_str(&value[context_start..start]);
        }

        // Add the highlight and its closing tag
        let highlight_end = value[start..]
            .find("</b>")
            .map(|i| start + i + 4)
            .unwrap_or(start);
        result.push_str(&value[start..highlight_end]);

        // Add text after highlight
        let context_end = if highlight_end + 20 < value.len() {
            // Find the character boundary 20 chars after
            let mut chars = value[highlight_end..].chars().take(20);
            let mut count = 0;
            let mut pos = highlight_end;
            while let Some(_) = chars.next() {
                pos = value[pos..]
                    .char_indices()
                    .next()
                    .map(|(i, _)| pos + i)
                    .unwrap_or(value.len());
                count += 1;
                if count >= 20 {
                    break;
                }
            }
            pos
        } else {
            value.len()
        };

        if context_end > highlight_end {
            result.push_str(&value[highlight_end..context_end]);
        }

        last_end = context_end;
    }

    result
}
