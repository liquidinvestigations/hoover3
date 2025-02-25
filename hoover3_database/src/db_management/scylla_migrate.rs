use std::collections::BTreeMap;

use super::ScyllaDatabaseHandle;
use crate::db_management::with_redis_cache;
use crate::db_management::DatabaseSpaceManager;
use anyhow::Result;
use hoover3_types::db_schema::DatabaseColumn;
use hoover3_types::db_schema::DatabaseColumnType;
use hoover3_types::db_schema::DatabaseTable;
use hoover3_types::db_schema::ScyllaDatabaseSchema;
use hoover3_types::identifier::CollectionId;
use hoover3_types::identifier::DatabaseIdentifier;
use scylla::transport::topology::CqlType;
use scylla::transport::topology::NativeType;

/// Test that the code schema object works.
#[test]
fn test_get_scylla_code_schema_1() {
    use hoover3_types::db_schema::get_scylla_schema_from_inventory;
    let _schema = get_scylla_schema_from_inventory();
    println!("{:?}", _schema);
}

/// Test that the code schema is same as the db schema. They are parsed by different code paths,
/// so table order might be different. The db schema does not have the correct table field order.
#[tokio::test]
async fn test_get_scylla_code_schema_2() {
    use hoover3_types::db_schema::get_scylla_schema_from_inventory;
    let code_schema = get_scylla_schema_from_inventory();
    let c1 = CollectionId::new("test_get_scylla_code_schema_2").unwrap();
    let session = ScyllaDatabaseHandle::global_session().await.unwrap();
    session
        .create_space(&c1.database_name().unwrap())
        .await
        .unwrap();

    ScyllaDatabaseHandle::migrate_collection_space(&c1)
        .await
        .unwrap();

    let db_schema = query_scylla_schema(&c1).await.unwrap();
    session
        .drop_space(&c1.database_name().unwrap())
        .await
        .unwrap();

    assert_eq!(db_schema.tables.len(), code_schema.tables.len());

    for (db_table, code_table) in db_schema.tables.iter().zip(code_schema.tables.iter()) {
        assert_eq!(db_table.0, code_table.0);
        assert_eq!(db_table.1.name, code_table.1.name);
        assert_eq!(db_table.1.columns.len(), code_table.1.columns.len());

        let mut db_columns = db_table.1.columns.clone();
        let mut code_columns = code_table.1.columns.clone();

        db_columns.sort_by_key(|c| c.name.clone());
        code_columns.sort_by_key(|c| c.name.clone());

        println!("db_table: {:#?}", db_table.1);
        println!("code_table: {:#?}", code_table.1);

        for (db_column, code_column) in db_columns.iter().zip(code_columns.iter()) {
            assert_eq!(db_column.name, code_column.name);
            assert_eq!(db_column._type, code_column._type);
            assert_eq!(db_column.primary, code_column.primary);
        }
    }
}

/// API Client method to get the Scylla database schema for a collection.
pub async fn query_scylla_schema(c: &CollectionId) -> Result<ScyllaDatabaseSchema> {
    let c = c.clone();
    let c2 = c.clone();
    with_redis_cache(
        "query_scylla_schema",
        60,
        move |c| _query_scylla_schema(c.clone()),
        &c2,
    )
    .await
}

/// Query database schema without caching.
async fn _query_scylla_schema(c: CollectionId) -> Result<ScyllaDatabaseSchema> {
    tracing::info!("_query_scylla_schema {}", c.to_string());

    let session = ScyllaDatabaseHandle::collection_session(&c).await?;
    let session = session.get_session();
    // https://rust-driver.docs.scylladb.com/stable/schema/schema.html
    session.refresh_metadata().await?;
    let cluster_data = &session.get_cluster_data();
    let keyspaces = &cluster_data.get_keyspace_info();
    use anyhow::Context;
    let keyspace = keyspaces
        .get(c.database_name()?.to_string().as_str())
        .context("keyspace not found")?;
    let tables = &keyspace.tables;
    let tables = tables
        .iter()
        .filter_map(|(table_name, table)| {
            let table_name = DatabaseIdentifier::new(table_name).ok();
            if let Some(table_name) = table_name {
                Some((
                    table_name.clone(),
                    DatabaseTable {
                        name: table_name.clone(),
                        columns: table
                            .columns
                            .iter()
                            .filter_map(|(column_name_str, column)| {
                                let column_name = DatabaseIdentifier::new(column_name_str).ok();
                                if let Some(column_name) = column_name {
                                    Some(DatabaseColumn {
                                        name: column_name.clone(),
                                        _type: convert_scylla_topology_column_type(&column.type_)
                                            .ok()?,
                                        primary: table.partition_key.contains(column_name_str)
                                            || table.clustering_key.contains(column_name_str),
                                    })
                                } else {
                                    None
                                }
                            })
                            .collect(),
                    },
                ))
            } else {
                None
            }
        })
        .collect();

    Ok(ScyllaDatabaseSchema { tables })
}

fn convert_scylla_topology_column_type(c: &CqlType) -> Result<DatabaseColumnType> {
    match c {
        CqlType::Native(NativeType::Ascii) => Ok(DatabaseColumnType::String),
        CqlType::Native(NativeType::Text) => Ok(DatabaseColumnType::String),
        CqlType::Native(NativeType::BigInt) => Ok(DatabaseColumnType::Int64),
        CqlType::Native(NativeType::Boolean) => Ok(DatabaseColumnType::Boolean),
        CqlType::Native(NativeType::Double) => Ok(DatabaseColumnType::Double),
        CqlType::Native(NativeType::Float) => Ok(DatabaseColumnType::Float),
        CqlType::Native(NativeType::Int) => Ok(DatabaseColumnType::Int32),
        CqlType::Native(NativeType::SmallInt) => Ok(DatabaseColumnType::Int16),
        CqlType::Native(NativeType::TinyInt) => Ok(DatabaseColumnType::Int8),
        CqlType::Native(NativeType::Timestamp) => Ok(DatabaseColumnType::Timestamp),
        CqlType::UserDefinedType {
            frozen: _,
            definition,
        } => {
            let definition = definition.as_ref().expect("missing UDT from db");
            Ok(DatabaseColumnType::Object(
                definition
                    .field_types
                    .iter()
                    .map(|(name, typ)| -> Result<(String, Box<DatabaseColumnType>)> {
                        Ok((
                            name.to_string(),
                            Box::new(convert_scylla_topology_column_type(typ)?),
                        ))
                    })
                    .collect::<Result<BTreeMap<String, Box<DatabaseColumnType>>>>()?,
            ))
        }
        _ => Err(anyhow::anyhow!("unsupported column type: {:?}", c)),
    }
}

// async fn query_scylla_table_schema(
//     c: &CollectionId,
//     table_name: &DatabaseIdentifier,
// ) -> Result<DatabaseTable> {
//     let session = ScyllaDatabaseHandle::global_session().await?;
//     let mut table = DatabaseTable {
//         name: table_name.clone(),
//         columns: vec![],
//     };
//     let ks_name = c.database_name()?.to_string();

//     let mut rows = vec![];

//     for row in session
//         .execute_unpaged(
//             "SELECT column_name, kind, position, type
//             FROM system_schema.columns
//             WHERE keyspace_name = ? AND table_name = ?",
//             (ks_name, table_name.to_string()),
//         )
//         .await?
//         .into_rows_result()?
//         .rows::<(String, String, i32, String)>()?
//     {
//         let (column_name, column_kind, column_position, column_type) = row?;
//         rows.push((
//             column_name.to_string(),
//             column_kind.to_string(),
//             column_position,
//             column_type.to_string(),
//         ));
//     }

//     // sort by kind, then position
//     rows.sort_by(|a, b| {
//         let _kind_pos_a = match a.1.as_str() {
//             "partition_key" => 0,
//             "clustering" => 1,
//             "regular" => 2,
//             _ => panic!("invalid column kind: {}", a.1),
//         };
//         let _kind_pos_b = match b.1.as_str() {
//             "partition_key" => 0,
//             "clustering" => 1,
//             "regular" => 2,
//             _ => panic!("invalid column kind: {}", b.1),
//         };

//         (_kind_pos_a, a.2).cmp(&(_kind_pos_b, b.2))
//     });

//     for (column_name, column_kind, _column_position, column_type) in rows {
//         if let Ok(column_name) = DatabaseIdentifier::new(&column_name) {
//             let primary = matches!(column_kind.as_str(), "partition_key" | "clustering");
//             table.columns.push(DatabaseColumn {
//                 name: column_name,
//                 _type: DatabaseColumnType::from_scylla_type(&column_type).unwrap(),
//                 primary,
//             });
//         } else {
//             info!("skipped scylla column {}", column_name);
//         }
//     }

//     Ok(table)
// }
