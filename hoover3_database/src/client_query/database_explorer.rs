use charybdis::scylla::{CqlValue, Row};
use hoover3_types::{
    db_schema::{
        DatabaseColumnType, DatabaseType, DatabaseValue, DynamicQueryResponse, DynamicQueryResult,
    },
    identifier::{CollectionId, DatabaseIdentifier},
};
use scylla::{
    frame::response::result::ColumnType,
    transport::{query_result::IntoRowsResultError, PagingState, PagingStateResponse},
    QueryResult,
};

use crate::db_management::{with_redis_cache, DatabaseSpaceManager, ScyllaDatabaseHandle};

/// Get Scylla table row count by running SQL request `SELECT COUNT * FROM ...`.
/// Cache result in Redis for 60 seconds.
pub async fn scylla_row_count(
    (collection_id, table_name): (CollectionId, DatabaseIdentifier),
) -> anyhow::Result<i64> {
    with_redis_cache(
        "scylla_row_count",
        60,
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

pub async fn db_explorer_run_query(
    (collection_id, db_type, sql_query): (CollectionId, DatabaseType, String),
) -> anyhow::Result<DynamicQueryResponse> {
    match db_type {
        DatabaseType::Scylla => db_explorer_run_scylla_query((collection_id, sql_query)).await,
        DatabaseType::Nebula => db_explorer_run_nebula_query((collection_id, sql_query)).await,
        DatabaseType::Meilisearch => {
            db_explorer_run_meilisearch_query((collection_id, sql_query)).await
        }
    }
}

async fn db_explorer_run_nebula_query(
    (collection_id, sql_query): (CollectionId, String),
) -> anyhow::Result<DynamicQueryResponse> {
    anyhow::bail!(
        "not implemented, nebula collection_id: {:?}, sql_query: {:?}",
        collection_id,
        sql_query
    )
}

async fn db_explorer_run_meilisearch_query(
    (collection_id, sql_query): (CollectionId, String),
) -> anyhow::Result<DynamicQueryResponse> {
    anyhow::bail!(
        "not implemented, meilisearch collection_id: {:?}, sql_query: {:?}",
        collection_id,
        sql_query
    )
}

/// Run a Scylla query and return the result.
async fn db_explorer_run_scylla_query(
    (collection_id, sql_query): (CollectionId, String),
) -> anyhow::Result<DynamicQueryResponse> {
    let t0 = std::time::Instant::now();
    let client = ScyllaDatabaseHandle::collection_session(&collection_id).await?;
    let client_query = scylla::query::Query::new(sql_query.clone()).with_page_size(100);
    let result = client
        .execute_single_page(client_query, (), PagingState::default())
        .await;
    let t1 = std::time::Instant::now();
    let dt = t1.duration_since(t0).as_secs_f64();
    let next_page = match result.as_ref().ok().map(|r| r.1.clone()) {
        Some(PagingStateResponse::HasMorePages { state: next_page }) => {
            next_page.as_bytes_slice().map(|v| v.to_vec())
        }
        _ => None,
    };
    let result = result
        .map(|r| r.0)
        .map_err(|e| anyhow::anyhow!(format!("{:#?}", e)));
    let result = print_scylladb_result(result).map_err(|e| e.to_string());
    Ok(DynamicQueryResponse {
        db_type: DatabaseType::Scylla,
        query: sql_query.clone(),
        result,
        elapsed_seconds: dt,
        next_page,
    })
}

/// Convert a Scylla query result into a DynamicQueryResult.
fn print_scylladb_result(
    result: anyhow::Result<QueryResult>,
) -> anyhow::Result<DynamicQueryResult> {
    let mut result_rows = vec![];
    let mut result_columns = vec![];
    let result = result?;

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
    })
}

fn convert_scylla_column_type(c: &ColumnType) -> DatabaseColumnType {
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
