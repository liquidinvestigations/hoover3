//! Database explorer module that provides functionality to execute and process queries
//! across different database types (Scylla, Nebula, Meilisearch). Includes utilities
//! for query execution, result conversion, and row counting.

use std::collections::BTreeMap;

use charybdis::scylla::{CqlValue, Row};
use hoover3_types::{
    db_schema::{
        DatabaseColumnType, DatabaseServiceType, DatabaseValue, DynamicQueryResponse,
        DynamicQueryResult,
    },
    identifier::{CollectionId, DatabaseIdentifier},
};
use scylla::{
    frame::response::result::ColumnType,
    transport::{query_result::IntoRowsResultError, PagingState, PagingStateResponse},
    QueryResult,
};

use crate::db_management::{
    redis::with_redis_cache, DatabaseSpaceManager, MeilisearchDatabaseHandle, ScyllaDatabaseHandle,
};

/// Get Scylla table row count by running SQL request `SELECT COUNT * FROM ...`.
/// Cache result in Redis for 60 seconds.
pub async fn scylla_row_count(
    (collection_id, table_name): (CollectionId, DatabaseIdentifier),
) -> anyhow::Result<i64> {
    with_redis_cache(
        "scylla_row_count",
        360,
        move |(collection_id, table_name)| async move {
            let client = ScyllaDatabaseHandle::collection_session(&collection_id).await?;
            let client_query =
                scylla::query::Query::new(format!("SELECT COUNT(*) FROM {table_name}"));
            let row_count = client.execute_unpaged(client_query, ()).await?;
            let row_count = row_count.into_rows_result()?;
            let row_count = row_count.first_row::<(i64,)>()?;
            Ok(row_count.0)
        },
        &(collection_id, table_name),
    )
    .await
}

/// Execute a query against the specified database type (Scylla, Nebula, or Meilisearch)
/// and return the results in a standardized DynamicQueryResponse format.
pub async fn db_explorer_run_query(
    (collection_id, db_type, sql_query): (CollectionId, DatabaseServiceType, String),
) -> anyhow::Result<DynamicQueryResponse> {
    let start_time = std::time::Instant::now();

    let result = match db_type {
        DatabaseServiceType::Scylla => {
            db_explorer_run_scylla_query((collection_id, sql_query.clone())).await
        }

        DatabaseServiceType::Meilisearch => {
            db_explorer_run_meilisearch_query((collection_id, sql_query.clone())).await
        }
    }
    .map_err(|e| format!("{:?} Query Error: {}", db_type, e));

    let t1 = std::time::Instant::now();
    let dt = t1.duration_since(start_time).as_secs_f64();
    let result_serialized_size_bytes = bincode::serialize(&result)?.len() as u64;
    Ok(DynamicQueryResponse {
        db_type,
        query: sql_query.clone(),
        result,
        elapsed_seconds: dt,
        result_serialized_size_bytes,
    })
}

async fn db_explorer_run_meilisearch_query(
    (collection_id, sql_query): (CollectionId, String),
) -> anyhow::Result<DynamicQueryResult> {
    let session = MeilisearchDatabaseHandle::collection_session(&collection_id).await?;
    let result = session
        .search()
        .with_query(&sql_query)
        .with_hits_per_page(100)
        .execute::<serde_json::Value>()
        .await?;

    let result = result.hits;
    if result.is_empty() {
        return Ok(DynamicQueryResult {
            columns: vec![],
            rows: vec![],
            next_page: None,
        });
    }
    let mut column_map = std::collections::BTreeMap::new();
    for hit in result.iter() {
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
    let rows = result
        .into_iter()
        .map(|r| match r.result {
            serde_json::Value::Object(o) => {
                let mut pairs = o
                    .into_iter()
                    .map(|(_k, v)| (_k, json_value_to_database_value(v)))
                    .collect::<BTreeMap<_, _>>();
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

    Ok(DynamicQueryResult {
        columns: column_map.into_iter().collect::<Vec<_>>(),
        rows,
        next_page: None,
    })
}

pub(crate) fn json_value_to_database_type(v: &serde_json::Value) -> Option<DatabaseColumnType> {
    match v {
        serde_json::Value::String(_) => Some(DatabaseColumnType::String),
        serde_json::Value::Number(_n) => {
            if _n.is_f64() {
                Some(DatabaseColumnType::Double)
            } else {
                Some(DatabaseColumnType::Int64)
            }
        }
        serde_json::Value::Bool(_) => Some(DatabaseColumnType::Boolean),
        serde_json::Value::Array(_v) => Some(DatabaseColumnType::List(Box::new(
            json_value_to_database_type(_v.first()?)?,
        ))),
        serde_json::Value::Object(_o) => {
            let columns = _o
                .iter()
                .filter_map(|(k, v)| {
                    Some((k.to_string(), Box::new(json_value_to_database_type(v)?)))
                })
                .collect::<BTreeMap<_, _>>();
            Some(DatabaseColumnType::Object(columns))
        }
        serde_json::Value::Null => None,
    }
}

pub(crate) fn json_value_to_database_value(v: serde_json::Value) -> Option<DatabaseValue> {
    match v {
        serde_json::Value::String(s) => Some(DatabaseValue::String(s.to_string())),
        serde_json::Value::Number(n) => {
            if n.is_f64() {
                Some(DatabaseValue::Double(n.as_f64().unwrap()))
            } else {
                Some(DatabaseValue::Int64(n.as_i64().unwrap()))
            }
        }
        serde_json::Value::Bool(b) => Some(DatabaseValue::Boolean(b)),
        serde_json::Value::Array(a) => Some(DatabaseValue::List(
            a.into_iter()
                .filter_map(json_value_to_database_value)
                .collect(),
        )),
        serde_json::Value::Object(o) => Some(DatabaseValue::Object(
            o.into_iter()
                .map(|(k, v)| (k.to_string(), json_value_to_database_value(v)))
                .collect(),
        )),
        _ => None,
    }
}

/// Run a Scylla query and return the result.
async fn db_explorer_run_scylla_query(
    (collection_id, sql_query): (CollectionId, String),
) -> anyhow::Result<DynamicQueryResult> {
    let client = ScyllaDatabaseHandle::collection_session(&collection_id).await?;
    let client_query = scylla::query::Query::new(sql_query.clone()).with_page_size(100);
    let result = client
        .execute_single_page(client_query, (), PagingState::default())
        .await?;
    let next_page = match result.1.clone() {
        PagingStateResponse::HasMorePages { state: next_page } => {
            next_page.as_bytes_slice().map(|v| v.to_vec())
        }
        _ => None,
    };
    print_scylladb_result(result.0, next_page)
}

/// Convert a Scylla query result into a DynamicQueryResult.
fn print_scylladb_result(
    result: QueryResult,
    next_page: Option<Vec<u8>>,
) -> anyhow::Result<DynamicQueryResult> {
    let mut result_rows = vec![];
    let mut result_columns = vec![];

    match result.into_rows_result() {
        Ok(rows_result) => {
            for column in rows_result.column_specs().iter() {
                let name = column.name().to_string();
                let typ = convert_scylla_column_type(column.typ());
                result_columns.push((name, typ));
            }
            for row in rows_result.rows::<Row>()? {
                let row = row?;
                result_rows.push(
                    row.columns
                        .into_iter()
                        .map(convert_scylla_column_value)
                        .collect(),
                );
            }
        }
        Err(IntoRowsResultError::ResultNotRows(_)) => {
            return DynamicQueryResult::from_single_string("OK".to_string())
        }
        Err(e) => return Err(e.into()),
    }

    Ok(DynamicQueryResult {
        columns: result_columns,
        rows: result_rows,
        next_page,
    })
}

/// Convert a Scylla column type to a dynamic DatabaseColumnType.
pub fn convert_scylla_column_type(c: &ColumnType) -> DatabaseColumnType {
    match c {
        ColumnType::Text => DatabaseColumnType::String,
        ColumnType::TinyInt => DatabaseColumnType::Int8,
        ColumnType::SmallInt => DatabaseColumnType::Int16,
        ColumnType::Int => DatabaseColumnType::Int32,
        ColumnType::BigInt => DatabaseColumnType::Int64,
        ColumnType::Float => DatabaseColumnType::Float,
        ColumnType::Double => DatabaseColumnType::Double,
        ColumnType::Boolean => DatabaseColumnType::Boolean,
        ColumnType::Timestamp => DatabaseColumnType::Timestamp,
        ColumnType::List(c) => DatabaseColumnType::List(Box::new(convert_scylla_column_type(c))),
        ColumnType::UserDefinedType {
            type_name: _,
            keyspace: _,
            field_types,
        } => DatabaseColumnType::Object(
            field_types
                .iter()
                .map(|(name, typ)| (name.to_string(), Box::new(convert_scylla_column_type(typ))))
                .collect(),
        ),
        _c => DatabaseColumnType::Other(format!("{:?}", _c)),
    }
}

fn convert_scylla_column_value(r: Option<CqlValue>) -> Option<DatabaseValue> {
    match r {
        None => None,
        Some(CqlValue::Text(s)) => Some(DatabaseValue::String(s)),
        Some(CqlValue::TinyInt(i)) => Some(DatabaseValue::Int8(i)),
        Some(CqlValue::SmallInt(i)) => Some(DatabaseValue::Int16(i)),
        Some(CqlValue::Int(i)) => Some(DatabaseValue::Int32(i)),
        Some(CqlValue::BigInt(i)) => Some(DatabaseValue::Int64(i)),
        Some(CqlValue::Float(f)) => Some(DatabaseValue::Float(f)),
        Some(CqlValue::Double(d)) => Some(DatabaseValue::Double(d)),
        Some(CqlValue::Boolean(b)) => Some(DatabaseValue::Boolean(b)),
        Some(CqlValue::Timestamp(t)) => {
            if let Ok(t) = t.try_into() {
                Some(DatabaseValue::Timestamp(t))
            } else {
                tracing::warn!("Timestamp Overflow: {:?}", t);
                None
            }
        }
        Some(CqlValue::List(l)) => Some(DatabaseValue::List(
            l.into_iter()
                .map(|v| convert_scylla_column_value(Some(v)))
                .collect::<Vec<Option<DatabaseValue>>>()
                .into_iter()
                .flatten()
                .collect(),
        )),
        Some(CqlValue::UserDefinedType {
            type_name: _,
            keyspace: _,
            fields,
        }) => Some(DatabaseValue::Object(
            fields
                .into_iter()
                .map(|(name, value)| (name.to_string(), convert_scylla_column_value(value)))
                .collect(),
        )),
        _ => Some(DatabaseValue::Other(format!("{:?}", r))),
    }
}
