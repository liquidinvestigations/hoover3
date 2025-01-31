use crate::db_management::DatabaseSpaceManager;
use crate::migrate::ScyllaDatabaseHandle;
use anyhow::Result;
use hoover3_types::identifier::CollectionId;
use hoover3_types::identifier::DatabaseIdentifier;
use std::collections::BTreeMap;
use tracing::info;

#[derive(Debug, Clone)]
pub struct DatabaseSchema {
    pub tables: BTreeMap<DatabaseIdentifier, DatabaseTable>,
}

#[derive(Debug, Clone)]
pub struct DatabaseTable {
    pub name: DatabaseIdentifier,
    pub columns: Vec<DatabaseColumn>,
}

#[derive(Debug, Clone)]
pub struct DatabaseColumn {
    pub name: DatabaseIdentifier,
    pub _type: String,
    pub primary: bool,
}

pub async fn get_scylla_schema_primary(c: &CollectionId) -> Result<DatabaseSchema> {
    let session = ScyllaDatabaseHandle::global_session().await?;
    let mut schema = DatabaseSchema {
        tables: BTreeMap::new(),
    };
    let ks_name = c.database_name()?.to_string();

    for row in session
        .execute_unpaged(
            "SELECT table_name
            FROM system_schema.tables
            WHERE keyspace_name = ?",
            (&ks_name,),
        )
        .await?
        .into_rows_result()?
        .rows::<(String,)>()?
    {
        let name = row?.0;
        if let Ok(name) = DatabaseIdentifier::new(&name) {
            schema.tables.insert(
                name.clone(),
                get_scylla_table_schema_primary(c, &name).await?,
            );
        } else {
            info!("skipped scylla table {}", name);
        }
    }

    Ok(schema)
}

async fn get_scylla_table_schema_primary(
    c: &CollectionId,
    table_name: &DatabaseIdentifier,
) -> Result<DatabaseTable> {
    let session = ScyllaDatabaseHandle::global_session().await?;
    let mut table = DatabaseTable {
        name: table_name.clone(),
        columns: vec![],
    };
    let ks_name = c.database_name()?.to_string();

    let mut rows = vec![];

    for row in session
        .execute_unpaged(
            "SELECT column_name, kind, position, type
            FROM system_schema.columns
            WHERE keyspace_name = ? AND table_name = ?",
            (ks_name, table_name.to_string()),
        )
        .await?
        .into_rows_result()?
        .rows::<(String, String, i32, String)>()?
    {
        let (column_name, column_kind, column_position, column_type) = row?;
        rows.push((
            column_name.to_string(),
            column_kind.to_string(),
            column_position,
            column_type.to_string(),
        ));
    }

    // sort by kind, then position
    rows.sort_by(|a, b| {
        let _kind_pos_a = match a.1.as_str() {
            "partition_key" => 0,
            "clustering" => 1,
            "regular" => 2,
            _ => panic!("invalid column kind: {}", a.1),
        };
        let _kind_pos_b = match b.1.as_str() {
            "partition_key" => 0,
            "clustering" => 1,
            "regular" => 2,
            _ => panic!("invalid column kind: {}", b.1),
        };

        ((_kind_pos_a, a.2)).cmp(&(_kind_pos_b, b.2))
    });

    for (column_name, column_kind, _column_position, column_type) in rows {
        if let Ok(column_name) = DatabaseIdentifier::new(&column_name) {
            let primary = matches!(column_kind.as_str(), "partition_key" | "clustering");
            if primary {
                table.columns.push(DatabaseColumn {
                    name: column_name,
                    _type: column_type,
                    primary,
                });
            }
        } else {
            info!("skipped scylla column {}", column_name);
        }
    }

    Ok(table)
}
